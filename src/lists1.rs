// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;
use std::collections::LinkedList;
use std::collections::linked_list::IterMut;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 100000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 1000000;

fn iota(iter: IterMut<'_, i32>, v: i32) {
    let mut i = 0;
    for e in iter {
        *e = i + v;
        i += 1;
    }
}

fn list_print_n(list: &LinkedList<i32>, n: i32) {
    let lastc = n - 1;
    let mut c = 0;
    for elem in list.iter() {
        if c >= n {
            break;
        }
        print!("{}", elem);
        if c < lastc {
            print!(" ");
        }
        c += 1;
    }
    println!();
}

fn list_reverse<T>(list: LinkedList<T>) -> LinkedList<T> {
    let mut reversed = LinkedList::new();
    for elem in list {
        reversed.push_front(elem);
    }
    return reversed
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        if args.nth(1).unwrap().parse::<i32>().unwrap() < 1 {
            1
        } else {
            args.nth(1).unwrap().parse::<i32>().unwrap()
        }
    } else {
        LENGTH
    };

    let mut b = LinkedList::<i32>::new();

    let mut a = LinkedList::<i32>::new();
    for _i in 0..n {
        a.push_back(Default::default());
    }
    iota(a.iter_mut(), 1);

    while a.len() > 0 {
        b.push_front(a.pop_front().unwrap());
    }

    list_print_n(&b, 2);

    b = list_reverse(b);

    println!("{}", if !b.contains(&0) { "false" } else { "true" });

    println!("{}", if !b.contains(&n) { "false" } else { "true" });

    let mid = n / 2;
    for elem in b.iter() {
        if *elem < mid {
            a.push_back(*elem);
        }
    }

    list_print_n(&a, 10);

    let mut sum = 0;
    for elem in b.iter() {
        if *elem < 1000 {
            sum += *elem;
        }
    }
    println!("{}", sum);

    a.append(&mut b);

    println!("{} {}", a.len(), *a.back().unwrap());
}
