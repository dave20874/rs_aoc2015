use md5;

pub struct Mining {
    prefix: String,
}

impl Mining {
    pub fn new(s: &str) -> Mining {
        Mining { prefix: s.to_string() }
    }

    fn get_soln(&self, zeros: usize) -> u32 {
        let mut n = 0;
        let target_prefix = "0".repeat(zeros);
        loop {
            // Construct string to hash
            let s = format!("{}{}", self.prefix,n.to_string());

            // Compute hash
            let digest = md5::compute(s);
            // println!("{}: {:x}", n, digest);

            // Check for solution criteria
            if format!("{:x}", digest)[0..zeros] == target_prefix {
                // Stop the search and return n
                break n
            }

            n += 1;
        }
    }
}

impl super::Day for Mining {
    fn part1(&mut self) -> Result<i64, &str> {
        return Ok(self.get_soln(5) as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        return Ok(self.get_soln(6) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_examples() {
        let examples: Vec<(&str, usize, u32)> = vec![
            ("abcdef", 5, 609043),
            ("pqrstuv", 5, 1048970),
        ];

        for (s, zeros, n) in examples {
            let day = Mining::new(s);
            assert_eq!(day.get_soln(zeros), n);
        }
    }

    #[test]
    fn test_part1() {
        let mut day = Mining::new("iwrupvqb");
        assert_eq!(day.part1(), Ok(346386))
    }


    #[test]
    fn test_part2() {
        let mut day = Mining::new("iwrupvqb");
        assert_eq!(day.part2(), Ok(9958218))
    }
}