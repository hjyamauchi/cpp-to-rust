// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
// from Bill Lear
use std::env;
use std::collections::LinkedList;
use std::collections::linked_list::IterMut;

#[cfg(feature = "small_problem_size")]
const SIZE: usize = 1000;

#[cfg(not(feature = "small_problem_size"))]
const SIZE: usize = 10000;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 300;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 3000;

fn iota(iter: IterMut<'_, usize>, v: usize) {
    let mut i = 0;
    for e in iter {
        *e = i + v;
        i += 1;
    }
}

fn list_reverse<T>(list: LinkedList<T>) -> LinkedList<T> {
    let mut reversed = LinkedList::new();
    for elem in list {
        reversed.push_front(elem);
    }
    return reversed
}

fn test_lists() -> usize {
    let mut li1 = LinkedList::<usize>::new();
    for _i in 0..SIZE {
        li1.push_back(Default::default());
    }

    iota(li1.iter_mut(), 1);

    let mut li2 = li1.clone();

    let mut li3 = LinkedList::<usize>::new();

    let mut n = li2.len();
    while n != 0 {
        n -= 1;
        li3.push_back(li2.pop_front().unwrap());
    }

    n = li3.len();
    while n != 0 {
        n -= 1;
        li2.push_back(li3.pop_back().unwrap());
    }

    li1 = list_reverse(li1);

    return if *li1.front().unwrap() == SIZE && li1.eq(&li2) {
        li1.len()
    } else {
        0 as usize
    };
}

fn main() {
    let mut args = env::args();
    let mut iter = if args.len() == 2 {
        if args.nth(1).unwrap().parse::<i32>().unwrap() < 1 {
            1
        } else {
            args.nth(1).unwrap().parse::<i32>().unwrap()
        }
    } else {
        LENGTH
    };

    let mut result: usize = 0;
    while iter > 0 {
        result = test_lists();
        iter -= 1;
    }

    println!("{}", result);
}

