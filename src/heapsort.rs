// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: usize = 800000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: usize = 8000000;

const IM: i64 = 139968;
const IA: i64 = 3877;
const IC: i64 = 29573;

fn get_random(last_random: &mut i64, max: f64) -> f64 {
    let new_last = (*last_random * IA + IC) % IM;
    *last_random = new_last;
    return max * (new_last as f64) / (IM as f64);
}

fn heapsort(n: usize, ra: &mut Vec<f64>) {
    let mut i: usize;
    let mut j: usize;
    let mut ir = n;
    let mut l = (n >> 1) + 1;
    let mut rra: f64;

    loop {
        if l > 1 {
            l -= 1;
            rra = ra[l];
        } else {
            rra = ra[ir];
            ra[ir] = ra[1];
            ir -= 1;
            if ir == 1 {
                ra[1] = rra;
                return;
            }
        }
        i = l;
        j = l << 1;
	while j <= ir {
	    if j < ir && ra[j] < ra[j+1] { j += 1; }
	    if rra < ra[j] {
		ra[i] = ra[j];
                i = j;
		j += i;
	    } else {
		j = ir + 1;
	    }
	}
	ra[i] = rra;
    }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<usize>().unwrap()
    } else {
        LENGTH
    };

    let mut last_random: i64 = 42;

    let mut ary = Vec::<f64>::new();
    ary.resize_with(n + 1, Default::default);
    for i in 1..=n {
        ary[i] = get_random(&mut last_random, 1.0);
    }

    heapsort(n, &mut ary);

    println!("{:.10}", ary[n]);
}
