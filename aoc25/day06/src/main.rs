use std::{fs::File, io::Read};

struct Problem {
    nums: Vec<Vec<i64>>,
    ops: Vec<char>,
}

impl Problem {
    fn build(f: &mut File) -> Problem {
        let mut nums = vec![];
        let mut ops = vec![];
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);

        for line in content.lines() {
            if line.contains("1") || line.contains("2") {
                let line = line
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().expect("Yes it is a list of numbers"))
                    .collect::<Vec<i64>>();
                nums.push(line);
            } else {
                let line = line
                    .split_whitespace()
                    .map(|c| c.parse::<char>().expect("Ye its a char * or +"))
                    .collect::<Vec<char>>();
                ops = line;
            }
        }

        Problem { nums, ops }
    }

    fn solve_p1(&self) -> i64 {
        let mut answer = 0;
        let number_of_operations = self.ops.len();
        let number_of_operands = self.nums.len();
        println!("{} {}", number_of_operations, number_of_operands);
        for i in 0..number_of_operations {
            if self.ops[i] == '*' {
                let mut current_result = 1;
                for j in 0..number_of_operands {
                    current_result *= self.nums[j][i];
                }
                answer += current_result;
            } else {
                let mut current_result = 0;
                for j in 0..number_of_operands {
                    current_result += self.nums[j][i];
                }
                answer += current_result;
            }
        }

        answer
    }
}

fn main() {
    let mut file = File::open("./input").expect("its there ma man");
    let problem = Problem::build(&mut file);
    println!("{:?}", problem.nums);
    println!("{:?}", problem.ops);

    let ans1 = problem.solve_p1();
    println!("{}", ans1);
}
