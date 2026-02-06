use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(path: &Path) {
    let mut final_sum = 0i64;

    let file = File::open(path).expect("The file should exist at the root of the Cargo project");
    let buf = BufReader::new(file);
    let buf_iter = buf.split(b',');
    for range in buf_iter {
        let range = range.expect("We only get here as long as there was a next");
        let mut iter = range.splitn(2, |c| *c == b'-');
        if let (Some(low_idx), Some(high_idx)) = (iter.next(), iter.next()) {
            let low_idx = String::from_utf8(low_idx.to_vec())
                .expect("Promised in question that it is a number")
                .parse::<i64>()
                .expect("Input is well known");

            let mut high_idx = String::from_utf8(high_idx.to_vec())
                .expect("Promised in question that it is a number");
            if high_idx.contains('\n') {
                high_idx = high_idx[..high_idx.len() - 1].to_string();
            }

            let high_idx = high_idx.parse::<i64>().expect("Input is well known");

            // println!("{low_idx} - {high_idx}");

            for s in low_idx..=high_idx {
                let s = s.to_string();
                if s.len() % 2 != 0 {
                    continue;
                }

                if s[..(s.len() / 2)] == s[(s.len() / 2)..] {
                    // println!("Invalid id: {s}");
                    let to_add = s
                        .parse::<i64>()
                        .expect("Know for sure in input this is number");

                    final_sum += to_add;
                }
            }
        }
    }
    println!("{}", final_sum);
}

fn part2(path: &Path) {
    let mut final_sum = 0i64;

    let file = File::open(path).expect("The file should exist at the root of the Cargo project");
    let buf = BufReader::new(file);
    let buf_iter = buf.split(b',');
    for range in buf_iter {
        let range = range.expect("We only get here as long as there was a next");
        let mut iter = range.splitn(2, |c| *c == b'-');
        if let (Some(low_idx), Some(high_idx)) = (iter.next(), iter.next()) {
            let low_idx = String::from_utf8(low_idx.to_vec())
                .expect("Promised in question that it is a number")
                .parse::<i64>()
                .expect("Input is well known");

            let mut high_idx = String::from_utf8(high_idx.to_vec())
                .expect("Promised in question that it is a number");
            if high_idx.contains('\n') {
                high_idx = high_idx[..high_idx.len() - 1].to_string();
            }

            let high_idx = high_idx.parse::<i64>().expect("Input is well known");

            // println!("{low_idx} - {high_idx}");

            'over_string: for s in low_idx..=high_idx {
                let mut s = s.to_string();

                'counter: for i in 1..=s.len() / 2 {
                    if s.len() % (i) != 0 {
                        // println!("skipper {} for {}", s, i + 1);
                        continue 'counter;
                    }
                    unsafe {
                        let s = s.as_mut_vec();
                        let mut s = s.chunks(i);
                        let mut pre = s.next().unwrap();
                        for n in s {
                            if n != pre {
                                continue 'counter;
                            }
                            // println!("{:?} = {:?} for {:?}", pre, n, s);
                            pre = n;
                        }
                    }

                    let to_add = s
                        .parse::<i64>()
                        .expect("Know for sure in input this is number");

                    final_sum += to_add;
                    continue 'over_string;
                }
            }
        }
    }
    println!("{}", final_sum);
}
// for 0..len if len % i + 1 == 0 verify if all slices are the same. If true, continue and add the number to final

fn main() {
    let path = Path::new("input");
    let path_test = Path::new("input_test");

    println!("------------ START TEST PART 1 ------------");
    part1(path_test);
    println!("------------ FINAL TEST PART 1 ------------");

    println!("------------ START REAL PART 1 ------------");
    part1(path);
    println!("------------ FINAL TEST PART 1 ------------");

    println!("------------ START TEST PART 2 ------------");
    part2(path_test);
    println!("------------ FINAL TEST PART 2 ------------");

    println!("------------ START REAL PART 2 ------------");
    part2(path);
    println!("------------ FINAL TEST PART 2 ------------");
}
