// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 40;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 43;

fn fib(n: u64) -> u64 {
    if n < 2 {
        return 1;
    } else {
        return fib(n - 2) + fib(n - 1);
    }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    println!("{}", fib(n as u64));
}
