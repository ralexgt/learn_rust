use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};
fn main() {
    let path = Path::new("input");
    let file = match File::open(path) {
        Err(e) => panic!("couldnt read file: {}", e),
        Ok(f) => f,
    };
    let buf = BufReader::new(file);

    let mut current_pos = 50;
    let mut count = 0;

    for line in buf.lines() {
        let line = line.unwrap();
        print!("Started at: {current_pos}, moved by {line}");
        let mut direction = line;
        let amount = direction.split_off(1).parse::<i32>().unwrap();

        let prev_count = count;

        if direction == "L" {
            for _ in 0..amount {
                current_pos -= 1;
                if current_pos == -1 {
                    current_pos = 99;
                }
                if current_pos == 0 {
                    count += 1;
                }
            }
        } else {
            for _ in 0..amount {
                current_pos += 1;
                if current_pos == 100 {
                    current_pos = 0;
                }
                if current_pos == 0 {
                    count += 1;
                }
            }
        }

        println!(
            " and now we are at {current_pos}. Passed 0 {0} times",
            count - prev_count
        );
    }

    println!("{count}");
}
