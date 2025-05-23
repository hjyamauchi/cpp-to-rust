// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 90000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 900000;

fn main() {
    let mut args = env::args();
    let n = 10 * if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut x = Vec::<i32>::new();
    x.resize(n as usize, 0);
    let mut y = Vec::<i32>::new();
    y.resize(n as usize, 0);

    let mut i: i32 = 0;
    while i < n {
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;

        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
        x[i as usize] = i; i += 1;
    }

    i = n - 1;
    while i >= 0 {
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;

        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
        y[i as usize] = x[i as usize]; i -= 1;
    }

    println!("{}", y.iter().last().unwrap());
}
