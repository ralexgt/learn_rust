use std::{fs::File, io::Read};

pub struct Runner {
    w: Worker,
}

impl Runner {
    pub fn new(mut file: File) -> Runner {
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        let lines = buf.split(|c| *c == b'\n').filter(|line| !line.is_empty());
        let mut matrix = Vec::new();
        for line in lines {
            matrix.push(line.to_vec());
        }

        Runner {
            w: Worker { matrix },
        }
    }

    pub fn run_p1(&self) -> i32 {
        self.w.solve_p1().0
    }

    pub fn run_p2(&mut self) -> i32 {
        self.w.solve_p2()
    }
}

struct Worker {
    matrix: Vec<Vec<u8>>,
}

impl Worker {
    fn solve_p1(&self) -> (i32, Vec<(usize, usize)>) {
        let mut answer = 0;
        let mut coords_to_clean = Vec::new();
        for i in 0..self.matrix[0].len() {
            for j in 0..self.matrix.len() {
                let mut adj = 0;
                if self.matrix[i][j] != 64 {
                    continue;
                }
                if i > 0 && self.matrix[i - 1][j] == 64 {
                    adj += 1;
                }
                if i + 1 < self.matrix[0].len() && self.matrix[i + 1][j] == 64 {
                    adj += 1;
                }
                if j > 0 && self.matrix[i][j - 1] == 64 {
                    adj += 1;
                }
                if j + 1 < self.matrix.len() && self.matrix[i][j + 1] == 64 {
                    adj += 1;
                }
                if i > 0 && j > 0 && self.matrix[i - 1][j - 1] == 64 {
                    adj += 1;
                }
                if i + 1 < self.matrix[0].len()
                    && j + 1 < self.matrix.len()
                    && self.matrix[i + 1][j + 1] == 64
                {
                    adj += 1;
                }
                if i + 1 < self.matrix[0].len() && j > 0 && self.matrix[i + 1][j - 1] == 64 {
                    adj += 1;
                }
                if i > 0 && j + 1 < self.matrix.len() && self.matrix[i - 1][j + 1] == 64 {
                    adj += 1;
                }

                if adj < 4 {
                    answer += 1;
                    coords_to_clean.push((i, j));
                }
            }
        }
        (answer, coords_to_clean)
    }

    fn solve_p2(&mut self) -> i32 {
        let mut final_answer = 0;
        loop {
            let (current_answer, coords_to_clean) = self.solve_p1();
            if current_answer == 0 {
                break;
            }
            final_answer += current_answer;
            for (i, j) in coords_to_clean {
                self.matrix[i][j] = 0;
            }
        }
        final_answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_ex() {
        let runner = Runner::new(File::open("input_test").expect("Yeah its there i swear"));
        assert_eq!(13, runner.run_p1());
    }

    #[test]
    fn p2_ex() {
        let mut runner = Runner::new(File::open("input_test").expect("Yeah its there i swear"));
        assert_eq!(43, runner.run_p2());
    }
}
