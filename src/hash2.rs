// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;
use std::collections::HashMap;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 200;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 2000;

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut hash1: HashMap<String, i32> = HashMap::new();
    let mut hash2: HashMap<String, i32> = HashMap::new();

    for i in 0..10000 {
        let s = format!("foo_{}", i);
        hash1.insert(s, i);
    }

    for _i in 0..n {
        for (key, value) in &hash1 {
            hash2.entry(key.to_string()).and_modify(|e| { *e += *value }).or_insert(*value);
        }
    }

    println!("{} {} {} {}", hash1["foo_1"], hash1["foo_9999"], hash2["foo_1"], hash2["foo_9999"]);
}
