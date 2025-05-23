// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 30;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 46;

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut x: i32 = 0;

    for _a in 0..n {
	for _b in 0..n {
	    for _c in 0..n {
		for _d in 0..n {
		    for _e in 0..n {
			for _f in 0..n {
                            x = x.wrapping_add(1);
                        }
                    }
                }
            }
        }
    }

    println!("{}", x);
}
