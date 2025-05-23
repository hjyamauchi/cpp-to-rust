// Adapted from https://github.com/llvm/llvm-test-suite and
// http://www.bagley.org/~doug/shootout/
use std::env;

#[cfg(feature = "small_problem_size")]
const LENGTH: i32 = 1000000;

#[cfg(not(feature = "small_problem_size"))]
const LENGTH: i32 = 70000000;

struct Toggle {
    state: bool
}

impl Toggle {
    fn new(start_state: bool) -> Toggle {
        Toggle { state: start_state }
    }
}

trait Togglable {
    fn activate(&mut self);
    fn value(&self) -> bool;
}

impl Togglable for Toggle {
    fn activate(&mut self) {
        self.state = !self.state;
    }
    fn value(&self) -> bool {
        return self.state;
    }
}

struct NthToggle {
    state: bool,
    count_max: i32,
    counter: i32,
}

impl NthToggle {
    fn new(start_state: bool, max_counter: i32) -> NthToggle {
        NthToggle {
            state: start_state,
            count_max: max_counter,
            counter: 0,
        }
    }
}

impl Togglable for NthToggle {
    fn activate(&mut self) {
        self.counter += 1;
	if self.counter >= self.count_max {
	    self.state = !self.state;
	    self.counter = 0;
	}
    }
    fn value(&self) -> bool {
        return self.state;
    }
}

fn main() {
    let mut args = env::args();
    let n = if args.len() == 2 {
        args.nth(1).unwrap().parse::<i32>().unwrap()
    } else {
        LENGTH
    };

    let mut toggle1 = Toggle::new(true);
    for _i in 0..5 {
        toggle1.activate();
        println!("{}", if toggle1.value() { "true" } else { "false" });
    }
    for _i in 0..n {
        let _toggle = Toggle::new(true);
    }

    println!();

    let mut ntoggle1 = NthToggle::new(true, 3);
    for _i in 0..8 {
        ntoggle1.activate();
        println!("{}", if ntoggle1.value() { "true" } else { "false" });
    }
    for _i in 0..n {
        let _ntoggle = NthToggle::new(true, 3);
    }
}
