// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 11;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 12;

fn ack(m: i32, n: i32) -> i32 {
    if m != 0 { ack(m - 1, if n != 0 { ack(m, n - 1) } else { 1 }) } else { n + 1 }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };
    println!("Ack(3,{}): {}", n, ack(3, n));
}
