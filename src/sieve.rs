// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

// Use VecDeque instead of LinkedList because Rust doesn't have stable
// ways to remove items from LinkedList and strongly encourage using
// Vec or VecDeque.
use std::collections::VecDeque;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 50;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 500;

fn sieve(unknown: &mut VecDeque::<i32>, primes: &mut Vec::<i32>) {
    while unknown.len() > 0 {
        let p = unknown.pop_front().unwrap();
        unknown.retain(|&x| x % p != 0);
        primes.push(p);
    }
}

fn main() {
    let mut args = env::args();
    let mut num = if args.len() == 2 {
        let arg1 = args.nth(1).unwrap().parse::<i32>().unwrap();
        if arg1 < 1 { 1 } else { arg1 }
    } else {
        LENGTH
    };
    let mut primes = Vec::<i32>::new();

    // run the sieve repeatedly
    while num != 0 {
        let mut integers = VecDeque::<i32>::new();
        for i in 2..8192 {
            integers.push_back(i);
        }
        primes.clear();
        sieve(&mut integers, &mut primes);
        num -= 1;
    }

    println!("Count: {}", primes.len());
}
