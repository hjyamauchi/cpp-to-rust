// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

const SIZE: usize = 30;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 10000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 100000;

fn mkmatrix(rows: usize, cols: usize) -> Vec::<Vec::<i32>> {
    let mut count = 1;
    let mut m = Vec::<Vec::<i32>>::new();
    m.resize(rows, Default::default());
    for i in 0..rows {
        m[i].resize(cols, Default::default());
        for j in 0..cols {
            m[i][j] = count;
            count += 1;
        }
    }
    return m;
}

fn mmult(rows: usize, cols: usize, m1: &Vec::<Vec::<i32>>,
         m2: &Vec::<Vec::<i32>>, m3: &mut Vec::<Vec::<i32>>) {
    for i in 0..rows {
        for j in 0..cols {
            let mut val = 0;
            for k in 0..cols {
                val += m1[i][k] * m2[k][j];
            }
            m3[i][j] = val;
        }
    }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let m1 = mkmatrix(SIZE, SIZE);
    let m2 = mkmatrix(SIZE, SIZE);
    let mut mm = mkmatrix(SIZE, SIZE);

    for _i in 0..n {
        mmult(SIZE, SIZE, &m1, &m2, &mut mm);
    }
    println!("{} {} {} {}", mm[0][0], mm[2][3], mm[3][2], mm[4][4]);
}
