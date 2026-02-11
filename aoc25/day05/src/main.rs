use std::fs::File;

use day05::Runner;

fn main() {
    let runner = Runner::new(File::open("input").expect("is there"));
    println!("{}", runner.run_p1());
    println!("{}", runner.run_p2());
}
