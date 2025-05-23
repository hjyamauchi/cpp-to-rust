// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;
use std::collections::HashMap;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 50000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 500000;

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut x = HashMap::<String, i32>::new();

    for i in 1..=n {
        let s = format!("{:X}", i);
        x.insert(s, i);
    }

    let mut c: i32 = 0;
    for i in (1..=n).rev() {
        let s = format!("{}", i);
        if x.contains_key(&s) {
            c += 1;
        }
    }

    println!("{}", c);
}
