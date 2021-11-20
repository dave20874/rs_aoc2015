use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub struct Houses {
    s: String,
}

impl Houses {
    pub fn load(filename: &str) -> Houses {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut s: String = String::new();

        reader.read_line(&mut s).unwrap();

        return Houses::from_str(&s)
    }

    pub fn from_str(s: &str) -> Houses {
        return Houses { s: s.to_string() };
    }

    pub fn visited(&self) -> usize {
        let mut visited: HashMap<(i32, i32), u32> = HashMap::new();
        let mut pos = (0, 0);
        visited.insert(pos, 1);

        for c in self.s.chars() {
            match c {
                '<' => pos.0 -= 1,
                '>' => pos.0 += 1,
                '^' => pos.1 += 1,
                'v' => pos.1 -= 1,
                _ => (),
            }

            *visited.entry(pos).or_insert(1) += 1;
        }

        return visited.len();
    }

    pub fn visited2(&self) -> usize {
        let mut visited: HashMap<(i32, i32), u32> = HashMap::new();
        let mut pos1 = (0, 0);
        let mut pos2 = (0, 0);
        visited.insert(pos1, 1);

        let mut turn = 1;
        for c in self.s.chars() {
            if turn == 1 {
                match c {
                    '<' => pos1.0 -= 1,
                    '>' => pos1.0 += 1,
                    '^' => pos1.1 += 1,
                    'v' => pos1.1 -= 1,
                    _ => (),
                }
                *visited.entry(pos1).or_insert(1) += 1;
                turn = 2;
            }
            else {
                match c {
                    '<' => pos2.0 -= 1,
                    '>' => pos2.0 += 1,
                    '^' => pos2.1 += 1,
                    'v' => pos2.1 -= 1,
                    _ => (),
                }
                *visited.entry(pos2).or_insert(1) += 1;
                turn = 1;
            }
        }

        return visited.len();
    }
}

impl super::Day for Houses {
    fn part1(&mut self) -> Result<i64, &str> {
        return Ok(self.visited() as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        return Ok(self.visited2() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_visited() {
        let examples: Vec<(&str, usize)> = vec![
            (">", 2),
            ("^>v<", 4),
            ("^v^v^v^v^v", 2),
        ];

        for (s, n) in examples {
            let houses = Houses::from_str(s);
            assert_eq!(houses.visited(), n);
        }
    }

    #[test]
    fn test_visited2() {
        let examples: Vec<(&str, usize)> = vec![
            ("^v", 3),
            ("^>v<", 3),
            ("^v^v^v^v^v", 11),
        ];

        for (s, n) in examples {
            let houses = Houses::from_str(s);
            assert_eq!(houses.visited2(), n);
        }
    }

    #[test]
    fn test_part1() {
        let mut day = Houses::load("data/day3_input.txt");
        assert_eq!(day.part1(), Ok(2565));
    }

    #[test]
    fn test_part2() {
        let mut day = Houses::load("data/day3_input.txt");
        assert_eq!(day.part2(), Ok(2639));
    }
}

