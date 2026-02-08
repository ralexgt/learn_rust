use memmap2::Mmap;
use std::fs::File;

struct Runner {
    file: File,
}

impl Runner {
    fn new(file: File) -> Runner {
        Runner { file }
    }

    fn part1(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let text = unsafe { Mmap::map(&self.file)? };
        text.advise(memmap2::Advice::Sequential)?;
        // get all non empty lines
        let text = text.split(|c| *c == b'\n').filter(|el| !el.is_empty());

        let mut total_sum = 0;

        for line in text {
            // let mut battery_bank = Vec::new();
            // for voltage in line {
            //     let voltage = *voltage as i8;
            //     battery_bank.push(voltage);
            // }

            let voltages: Vec<u32> = str::from_utf8(line)
                .expect("The file is well known")
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("The file is well known to only have numbers")
                })
                .collect();

            // Calculate the running max
            let mut current_max = 0;
            for i in 0..voltages.len() - 1 {
                for j in i + 1..voltages.len() {
                    let sum = voltages[i] * 10 + voltages[j];
                    if sum > current_max {
                        current_max = sum;
                    }
                }
            }

            total_sum += current_max;
            // println!("{voltages:?} - max is: {current_max}");
        }
        // println!("total: {}", total_sum);

        Ok(total_sum)
    }

    fn part2(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let text = unsafe { Mmap::map(&self.file)? };
        text.advise(memmap2::Advice::Sequential)?;
        // get all non empty lines
        let text = text.split(|c| *c == b'\n').filter(|el| !el.is_empty());
        let mut total_sum = 0;

        for line in text {
            // let mut battery_bank = Vec::new();
            // for voltage in line {
            //     let voltage = *voltage as i8;
            //     battery_bank.push(voltage);
            // }
            let mut current_total = 0;

            let voltages: Vec<u32> = str::from_utf8(line)
                .expect("The file is well known")
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("The file is well known to only have numbers")
                })
                .collect();

            let mut start = 0;
            for i in 0..12 {
                let limit = 11 - i;
                let Some((pos, &next_digit)) = voltages[start..voltages.len() - limit]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by(|a, b| a.1.cmp(b.1))
                else {
                    panic!("shouldnt be possible");
                };
                start += pos + 1;
                current_total = (current_total * 10) + next_digit;
            }
            total_sum += current_total;
        }

        // println!("total: {}", total_sum);

        Ok(total_sum)
    }
}

fn main() {
    let file = File::open("input").expect("The file does exist");

    let runner = Runner::new(file);
    let res1 = runner
        .part1()
        .expect("Should never fail with the predefined input");

    let test_file = File::open("input_test").expect("The file does exist");
    let test_runner = Runner::new(test_file);
    let res1_test = test_runner
        .part1()
        .expect("Should never fail for the predefined inputs");

    println!("PART 1: Test run: {}", res1_test);
    println!("PART 1: Real run: {}", res1);

    let file = File::open("input").expect("The file does exist");

    let runner = Runner::new(file);
    let res2 = runner
        .part2()
        .expect("Should never fail with the predefined input");

    let test_file = File::open("input_test").expect("The file does exist");
    let test_runner = Runner::new(test_file);
    let res2_test = test_runner
        .part2()
        .expect("Should never fail for the predefined inputs");

    println!("PART 2: Test run: {}", res2_test);
    println!("PART 2: Real run: {}", res2);
}
