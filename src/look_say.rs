pub struct LookSay {
    digits: String,
}

// Speed this up with Vec<usize> instead of string operations.
// Have step() return Vec<usize> instead of updating self.digits.

impl LookSay {
    pub fn new(start_digits: &str) -> LookSay {
        LookSay { digits: start_digits.to_string() }
    }

    pub fn step(&self, n: u32) -> Vec<usize> {
        let mut seq: Vec<usize> = Vec::new();
        for c in self.digits.chars() {
            seq.push(c.to_string().parse().unwrap());
        }

        for _ in 0..n {
            let mut next_seq: Vec<usize> = Vec::new();

            let mut digit = seq[0];
            let mut count = 0;
            for d in seq {
                if d == digit {
                    // one more of these digits we're seeing
                    count += 1;
                }
                else {
                    // a new digit! close out the old sequence
                    next_seq.push(count);
                    next_seq.push(digit);

                    // start new sequence
                    digit = d;
                    count = 1;
                }
            }
            // close out the last sequence
            next_seq.push(count);
            next_seq.push(digit);

            seq = next_seq;
        }

        seq
    }
}

impl super::Day for LookSay {
    fn part1(&mut self) -> Result<i64, &str> {
        let seq = self.step(40);
        return Ok(seq.len() as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        let seq = self.step(50);
        return Ok(seq.len() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_example() {
        let day = LookSay::new("1");
        assert_eq!(day.step(1), [1, 1]);
        day.step(1);
        assert_eq!(day.step(2), [2, 1]);
        day.step(1);
        assert_eq!(day.step(3), [1, 2, 1, 1]);
        day.step(1);
        assert_eq!(day.step(4), [1, 1, 1, 2, 2, 1]);
        day.step(1);
        assert_eq!(day.step(5), [3, 1, 2, 2, 1, 1]);
    }

    #[test]
    fn test_part1() {
        let mut day = LookSay::new("1113122113");
        assert_eq!(day.part1(), Ok(360154));
    }

    #[test]
    fn test_part2() {
        let mut day = LookSay::new("1113122113");
        assert_eq!(day.part2(), Ok(5103798));
    }
}