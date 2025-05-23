// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 4000000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 400000000;

const IM: i64 = 139968;
const IA: i64 = 3877;
const IC: i64 = 29573;

fn gen_random(last_random: &mut i64, max: f64) -> f64 {
    let new_last = (*last_random * IA + IC) % IM;
    *last_random = new_last;
    return max * (new_last as f64) / (IM as f64);
}

fn main() {
    let mut args = env::args();
    let mut n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut result: f64 = 0.0;
    let mut last_random: i64 = 42;

    while n != 0 {
        n -= 1;
        result = gen_random(&mut last_random, 100.0);
    }

    println!("{:.9}", result);
}
