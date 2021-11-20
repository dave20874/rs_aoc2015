use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct NaughtyNice {
    strings: Vec<String>,
}

impl NaughtyNice {
    pub fn load(filename: &str) -> NaughtyNice {
        let mut db = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            db.push(line.unwrap().trim().to_string());
        }

        NaughtyNice {strings: db}
    }

    fn is_nice(s: &str) -> bool {
        let mut vowels = 0;
        let mut pairs = 0;
        let mut toxic = false;

        let mut last_c = '~';
        for c in s.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if c == last_c {
                pairs += 1
            }
            last_c = c;
        }

        let bads = ["ab", "cd", "pq", "xy"];
        for bad_str in bads {
            if s.contains(bad_str) {
                toxic = true
            }
        }

        (vowels >= 3) & (pairs >= 1) & !toxic
    }


    fn is_nice2(s: &str) -> bool {
        let mut criteria1 = false;
        let mut criteria2 = false;

        let s_chars:Vec<char> = s.chars().collect();

        // Check for criteria 1
        'outer: for n in 0..s_chars.len()-3 {
            for m in n+2..s_chars.len()-1 {
                if s_chars[n] == s_chars[m] && s_chars[n+1] == s_chars[m+1] {
                    criteria1 = true;
                    break 'outer;
                }
            }
        }

        // Check for criteria 2
        for n in 0..s_chars.len()-2 {
            if s_chars[n] == s_chars[n+2] {
                criteria2 = true;
                break;
            }
        }

        criteria1 & criteria2
    }

    fn num_nice(&self) -> usize {
        let mut count = 0;

        for s in &self.strings {
            if NaughtyNice::is_nice(s) {
                count += 1;
            }
        }

        count
    }

    fn num_nice2(&self) -> usize {
        let mut count = 0;

        for s in &self.strings {
            if NaughtyNice::is_nice2(s) {
                count += 1;
            }
        }

        count
    }
}

impl super::Day for NaughtyNice {
    fn part1(&mut self) -> Result<i64, &str> {
        return Ok(self.num_nice() as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        return Ok(self.num_nice2() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let nn = NaughtyNice::load("data/day5_input.txt");
        assert_eq!(nn.strings.len(), 1000);
        assert_eq!(nn.strings[0].len(), 16);
    }

    #[test]
    fn test_examples() {
        let examples: Vec<(&str, bool)> = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for (s, expected) in examples {

            assert_eq!(NaughtyNice::is_nice(s), expected);
        }
    }

    #[test]
    fn test_examples2() {
        let examples: Vec<(&str, bool)> = vec![
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];

        for (s, expected) in examples {

            assert_eq!(NaughtyNice::is_nice2(s), expected);
        }
    }

    #[test]
    fn test_part1() {
        let mut day = NaughtyNice::load("data/day5_input.txt");
        assert_eq!(day.part1(), Ok(255));
    }


    #[test]
    fn test_part2() {
        let mut day = NaughtyNice::load("data/day5_input.txt");
        assert_eq!(day.part2(), Ok(55));
    }
}
