/*
   small prime sieve I, will generate 100_000_000 in slightly under 10 seconds on my computer (using --release)
   uses segmented sieve of eratosthenes to achive this.
   I also made a small prime number struct so that you can just index into it without caring about implementation
*/

use core::cmp::min;
use std::time;
fn main() {
    let mut prime_nums = Primes::new();
    let a = time::Instant::now();
    println!("{}", prime_nums.get(1_000_000_000));
    println!("{:?}", a.elapsed());
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
            add_next_seg(&mut self.prime_numbers, &mut self.iterations)
        }
        &self.prime_numbers[ind]
    }
}

//this const was fast on my computer, you should mess around with it
const MAX_JUMP: usize = 1;
fn add_next_seg(primes: &mut Vec<usize>, iterations: &mut usize) {
    let seg_size = min(MAX_JUMP, primes.len() - *iterations - 1);
    let start = *iterations;
    let end = start + seg_size;
    let seg_end = primes[end].pow(2);
    let seg_start = primes[start].pow(2);
    let mut primeness = vec![true; seg_end - seg_start];
    /*
       primeness is an array representing if the number in that relative index is prime
       start and end are the last prime numbers for wheech we compleated the segment
       seg_start, seg_end are the first and last numbers we will check in this iteration
    */
    for i in 0..=end {
        let known_prime = primes[i];
        for not_prime in
            (seg_start + (known_prime - seg_start % known_prime)..seg_end).step_by(known_prime)
        {
            primeness[not_prime - seg_start] = false;
        }
        if seg_start % known_prime == 0 {
            primeness[0] = false;
        }
    }
    primes.reserve(
        ((seg_end as f32 / ((seg_end as f32).ln()))
            - (seg_start as f32 / ((seg_start as f32).ln()))) as usize,
    );
    for i in (0..primeness.len()).step_by(2) {
        if primeness[i] {
            primes.push(seg_start + i)
        }
    }
    *iterations = end;
}