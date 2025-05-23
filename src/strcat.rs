// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
// with help from PeterB
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 1000000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 5000000;

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut str = String::new();
    let mut capacity: usize = 31;
    str.reserve(capacity);
    let mut new_length: usize = 6;
    for _i in 0..n {
        if new_length > capacity {
            capacity *= 2;
            str.reserve(capacity);
        }
        str.push_str("hello\n");
        new_length += 6;
    }
    println!("{}", str.len());
}
