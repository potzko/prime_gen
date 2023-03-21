/*
   when looking to optimize I tried multy threading,
   after benchmarking and profiling I saw that in actuality there was a preformance loss

   I think this is do to the entire calculation is done in ram, so any cpu speedup would get slowed down in the ram bottleneck 
   might be worth to look at on other computers
*/

use core::cmp::min;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time;
const PRIME_PRINT_COUNT: usize = 1;
fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let mut prime_nums = Primes::new();
    let a = time::Instant::now();
    println!("{}", prime_nums.get(10_000_000_000));
    println!("total time: {:?}", a.elapsed());
    println!(
        "found {} consecutive prime numbers in {} iterations\nlargest number found is: {}",
        prime_nums.prime_numbers.len(),
        prime_nums.iterations,
        prime_nums.prime_numbers[prime_nums.prime_numbers.len() - 1]
    );
    let len = prime_nums.prime_numbers.len();
    print!(
        "{:?}",
        &prime_nums.prime_numbers[len - PRIME_PRINT_COUNT..len]
    );
}

#[derive(Debug)]
pub struct Primes {
    prime_numbers: Vec<usize>,
    iterations: usize,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            // we simulate having done the first iteration so that each segment will start with an odd number
            prime_numbers: vec![2, 3, 5, 7],
            iterations: 1,
        }
    }
    fn get(&mut self, ind: usize) -> &usize {
        while !(ind < self.prime_numbers.len()) {
            /*println!(
                "found {} prime numbers in {} iterations\n, largest number found is: {}",
                self.prime_numbers.len(),
                self.iterations,
                self.prime_numbers[self.prime_numbers.len() - 1]
            );*/
            add_next_seg(&mut self.prime_numbers, &mut self.iterations)
        }
        &self.prime_numbers[ind]
    }
}



//the amount of segments to check in each alocation, 1 and 2 are fast for me but feel free to mess with it
//this idea might be more importent in interpreted languadges
const MAX_JUMP: usize = 1;
fn add_next_seg(primes: &mut Vec<usize>, iterations: &mut usize) {
    let seg_size = min(MAX_JUMP, primes.len() - *iterations - 1);
    let start = *iterations;
    let end = start + seg_size;
    let seg_end = primes[end].pow(2);
    let seg_start = primes[start].pow(2);
    let mut primeness = Vec::with_capacity(seg_end - seg_start);
    for _ in 0..seg_end - seg_start {
        primeness.push(AtomicBool::new(true));
    }
    /*
       primeness is an array representing if the number in that relative index is prime
       start and end are the last prime numbers for wheech we compleated the segment
       seg_start, seg_end are the first and last numbers we will check in this iteration
    */
    primes[0..end].par_iter().for_each(|&known_prime| {
        for not_prime in
            (seg_start + (known_prime - seg_start % known_prime)..seg_end).step_by(known_prime)
        {
            primeness[not_prime - seg_start].store(false, Ordering::Relaxed);
        }
        if seg_start % known_prime == 0 {
            primeness[0].store(false, Ordering::Relaxed);
        }
    });

    /*
       an aproximation for how many primes are in the segments
       primes between A and B (A<B) will about equal
       B/ln(B) - A/ln(A)

    primes.reserve(
        ((seg_end as f32 / ((seg_end as f32).ln()))
            - (seg_start as f32 / ((seg_start as f32).ln()))) as usize,
    );*/
    for i in (0..primeness.len()).step_by(2) {
        if primeness[i].load(Ordering::Relaxed) {
            primes.push(seg_start + i)
        }
    }
    *iterations = end;
}
