use std::{fs::File, io::Read};

type RangeMin = u64;
type RangeMax = u64;
type ID = u64;

pub struct Runner {
    w: Worker,
}

impl Runner {
    pub fn new(mut file: File) -> Runner {
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);
        // let text = buf.split()
        let mut sections = buf
            .split("\n\n")
            .filter(|section| !section.trim().is_empty());

        let ranges = sections
            .next()
            .expect("Based on the input provided there are exactly 2 sections")
            .lines()
            .map(|line| {
                let mut parts = line.split('-');

                let start = parts
                    .next()
                    .expect("Missing start")
                    .parse::<RangeMin>()
                    .expect("Invalid number");

                let end = parts
                    .next()
                    .expect("Missing end")
                    .parse::<RangeMax>()
                    .expect("Invalid number");

                (start, end)
            })
            .collect::<Vec<(RangeMin, RangeMax)>>();

        let ids = sections
            .next()
            .expect("Based on the input provided there are exactly 2 sections")
            .lines()
            .map(|num| {
                num.parse::<ID>()
                    .expect("Provided input for sure has only positive integers")
            })
            .collect::<Vec<ID>>();

        Runner {
            w: Worker { ranges, ids },
        }
    }

    pub fn run_p1(&self) -> u64 {
        self.w.solve_p1()
    }

    pub fn run_p2(&self) -> u64 {
        self.w.solve_p2()
    }
}

struct Worker {
    ranges: Vec<(RangeMin, RangeMax)>,
    ids: Vec<ID>,
}

impl Worker {
    fn solve_p1(&self) -> u64 {
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(|(a, _b)| *a);
        let mut compact: Vec<(RangeMin, RangeMax)> = Vec::with_capacity(ranges.len());
        for (a, b) in ranges {
            if let Some((_last_a, last_b)) = compact.last_mut()
                && a <= (*last_b).saturating_add(1)
            {
                if b > *last_b {
                    *last_b = b;
                }
                continue;
            }
            compact.push((a, b));
        }

        let mut count = 0u64;
        for &id in &self.ids {
            if self._in_any_range(&compact, id) {
                count += 1
            };
        }

        count
    }

    fn solve_p2(&self) -> u64 {
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(|(a, _b)| *a);
        let mut compact: Vec<(RangeMin, RangeMax)> = Vec::with_capacity(ranges.len());
        for (a, b) in ranges {
            if let Some((_last_a, last_b)) = compact.last_mut()
                && a <= (*last_b).saturating_add(1)
            {
                if b > *last_b {
                    *last_b = b;
                }
                continue;
            }
            compact.push((a, b));
        }

        let mut count = 0u64;
        for range in compact {
            count += range.1 - range.0 + 1;
        }

        count
    }

    fn _in_any_range(&self, ranges: &[(u64, u64)], x: u64) -> bool {
        // find last range with start <= x
        match ranges.binary_search_by(|(start, _end)| start.cmp(&x)) {
            Ok(i) => {
                // exactly at a start => definitely inside
                x <= ranges[i].1
            }
            Err(0) => false,
            Err(i) => {
                let (_s, e) = ranges[i - 1];
                x <= e
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_ex() {
        let runner = Runner::new(File::open("input_test").expect("Yeah its there i swear"));
        assert_eq!(3, runner.run_p1());
    }

    #[test]
    fn p2_ex() {
        let mut runner = Runner::new(File::open("input_test").expect("Yeah its there i swear"));
        assert_eq!(14, runner.run_p2());
    }
}
