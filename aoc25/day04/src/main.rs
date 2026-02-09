use std::fs::File;

use day04::Runner;

fn main() {
    let mut runner = Runner::new(File::open("input").expect("Yeah it is there"));
    println!("{}", runner.run_p1());
    println!("{}", runner.run_p2());
}
