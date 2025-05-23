// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 150000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 1500000;

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut x = Vec::<i32>::new();
    x.resize(n as usize, 0);
    let mut y = Vec::<i32>::new();
    y.resize(n as usize, 0);

    for i in 0..n {
        x[i as usize] = i + 1;
    }

    for _k in 0..1000 {
        for i in (0..n).rev() {
            y[i as usize] += x[i as usize];
        }
    }

    println!("{} {}", y[0], y.iter().last().unwrap());
}
