// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
// Calculate statistical moments of a region, from Bill Lear
// Modified by Tamás Benkõ
// Further modified by Tom Hyer
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 500000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 5000000;

struct Moments<T> {
    median: T,
    mean: T,
    average_deviation: T,
    standard_deviation: T,
    variance: T,
    skew: T,
    kurtosis: T,
}

impl Moments<f64> {
    fn new(slice: &mut [f64]) -> Moments<f64> {
        let mut m = Moments::<f64> {
            median: 0.0,
            mean: 0.0,
            average_deviation: 0.0,
            standard_deviation: 0.0,
            variance: 0.0,
            skew: 0.0,
            kurtosis: 0.0,
        };

        let sum: f64 = slice.iter().sum();
        let n = slice.iter().count();
        m.mean = sum / (n as f64);
        for e in slice.iter() {
            let deviation = *e - m.mean;
            m.average_deviation += deviation.abs();
            let mut temp = deviation * deviation;
            m.variance += temp;
            temp *= deviation;
            m.skew += temp;
            m.kurtosis += temp * deviation;
        }
        m.average_deviation /= n as f64;
        m.variance /= (n - 1) as f64;
        m.standard_deviation = m.variance.sqrt();

        if m.variance != 0.0 {
            m.skew /= (n as f64) * m.variance * m.standard_deviation;
            m.kurtosis =
                m.kurtosis / ((n as f64) * m.variance * m.variance) - 3.0;
        }

        let (before, mid, _after) =
            slice.select_nth_unstable_by(n / 2,
                                         |a, b| a.partial_cmp(b).unwrap());
        if n % 2 == 0 {
            let next_biggest =
                before.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
            m.median = (*mid + *next_biggest.unwrap()) / 2.0;
        } else {
	    m.median = *mid;
        }

        return m;
    }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };
    let mut v = Vec::<f64>::new();

    for i in 0..n {
        v.push(i as f64);
    }
    let m = Moments::<f64>::new(&mut v[..]);

    println!("n:                  {}", v.iter().count());
    println!("median:             {:.6}", m.median);
    println!("mean:               {:.6}", m.mean);
    println!("average_deviation:  {:.6}", m.average_deviation);
    println!("standard_deviation: {:.6}", m.standard_deviation);
    println!("variance:           {:.6}", m.variance);
    println!("skew:               {:.6}", m.skew);
    println!("kurtosis:           {:.6}", m.kurtosis);
}
