use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct NotQuiteLisp {
    s: String,
}

impl NotQuiteLisp {
    pub fn load(filename: &str) -> NotQuiteLisp {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut s: String = String::new();

        reader.read_line(&mut s).unwrap();

        return NotQuiteLisp::from_str(&s)
    }

    pub fn from_str(s: &str) -> NotQuiteLisp {
        return NotQuiteLisp { s: s.to_string() };
    }

    pub fn floor(&self) -> i32 {
        let mut n = 0;
        for c in self.s.chars() {
            match c {
                '(' => n += 1,
                ')' => n -= 1,
                _ => (),
            }
        }

        return n;
    }

    pub fn basement_pos(&self) -> i32 {
        let mut n = 0;
        let mut position = 0;
        for c in self.s.chars() {
            position += 1;
            match c {
                '(' => n += 1,
                ')' => n -= 1,
                _ => (),
            }
            if n < 0 {
                return position;
            }
        }

        return n;
    }

}

impl super::Day for NotQuiteLisp {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.floor() as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.basement_pos() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_floor() {
        let examples: Vec<(&str, i32)> = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (s, n) in examples {
            let nql = NotQuiteLisp::from_str(s);
            assert_eq!(nql.floor(), n);
        }
    }

    #[test]
    fn test_basement_pos() {
        let examples: Vec<(&str, i32)> = vec![
            (")", 1),
            ("()())", 5),
        ];

        for (s, n) in examples {
            let nql = NotQuiteLisp::from_str(s);
            assert_eq!(nql.basement_pos(), n);
        }
    }

    #[test]
    fn test_load() {}

    #[test]
    fn test_part1() {
        let rr = &NotQuiteLisp::load("data/day1_input.txt");
        assert_eq!(rr.part1(), Ok(138));
    }

    #[test]
    fn test_part2() {
        let rr = &NotQuiteLisp::load("data/day1_input.txt");
        assert_eq!(rr.part2(), Ok(1771));
    }
}