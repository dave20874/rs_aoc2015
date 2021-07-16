use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Matchsticks {
    strings: Vec<String>,
}

impl Matchsticks {
    pub fn load(filename: &str) -> Matchsticks {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut strings: Vec<String> = Vec::new();

        for line in reader.lines() {
            strings.push(line.unwrap().trim().to_string());
        }

        Matchsticks { strings: strings }
    }

    fn code_chars(s: &str) -> usize {
        s.len()
    }

    fn mem_chars(s: &str) -> usize {
        let char_array: Vec<char> = s.chars().collect();
        let l = char_array.len();
        let mut n = 0;
        let mut escaped = false;
        let mut num_mem_chars = 0;
        while n < l {
            let c = char_array[n];

            match escaped {
                true => {
                    match c {
                        '\\' => {
                            // an escaped backslash
                            num_mem_chars += 1;
                        }
                        '"' => {
                            // An escaped double quote
                            num_mem_chars += 1;
                        }
                        'x' => {
                            // A hex sequence, skip next two input chars.
                            num_mem_chars += 1;
                            n += 2;
                        }
                        _ => {
                            // Weird escaped char, count it as one.
                            num_mem_chars += 1;
                        }
                    }
                    escaped = false;
                }
                false => {
                    match c {
                        '\\' => {
                            // Escape sequence starts -- this doesn't go into mem.
                            escaped = true;
                        }
                        _ => {
                            num_mem_chars += 1;
                        }
                    }
                }
            }

            n += 1;
        }

        // minus two subtracts the first and last double quotes enclosing the string.
        num_mem_chars - 2
    }

    fn encoded_chars(s: &str) -> usize {
        let s_chars: Vec<char> = s.chars().collect();
        let mut s2: Vec<char> = Vec::new();

        s2.push('"');
        for c in s_chars {
            match c {
                '"' | '\\' => { s2.push('\\'); }
                _ => ()
            }
            s2.push(c);
        }
        s2.push('"');

        s2.len()
    }
}

impl super::Day for Matchsticks {
    fn part1(&self) -> Result<i64, &str> {
        let mut code = 0;
        let mut mem = 0;

        for s in &self.strings {
            code += Matchsticks::code_chars(&s);
            mem += Matchsticks::mem_chars(&s);
        }
        return Ok((code - mem) as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut code = 0;
        let mut encoded = 0;

        for s in &self.strings {
            code += Matchsticks::code_chars(&s);
            encoded += Matchsticks::encoded_chars(&s);
        }
        return Ok((encoded - code) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let day = Matchsticks::load("data/day8_example1.txt");

        assert_eq!(day.strings.len(), 4);
    }

    #[test]
    fn test_load_input() {
        let day = Matchsticks::load("data/day8_input.txt");

        assert_eq!(day.strings.len(), 300);
    }

    #[test]
    fn test_code_chars() {
        assert_eq!(Matchsticks::code_chars("\"\""), 2);
        assert_eq!(Matchsticks::code_chars("\"abc\""), 5);
        assert_eq!(Matchsticks::code_chars("\"\\\\\\\\\""), 6);
    }

    #[test]
    fn test_mem_chars() {
        assert_eq!(Matchsticks::mem_chars("\"\""), 0);
        assert_eq!(Matchsticks::mem_chars("\"abc\""), 3);
        assert_eq!(Matchsticks::mem_chars("\"\\\\\\\\\""), 2);
    }

    #[test]
    fn test_encoded_chars() {
        assert_eq!(Matchsticks::encoded_chars("\"\""), 6);
        assert_eq!(Matchsticks::encoded_chars("\"abc\""), 9);
        assert_eq!(Matchsticks::encoded_chars("\"\\\\\\\\\""), 14);
    }

    #[test]
    fn test_example() {
        let day = Matchsticks::load("data/day8_example1.txt");

        assert_eq!(day.part1(), Ok(12));
    }

    #[test]
    fn test_part1() {
        let day = Matchsticks::load("data/day8_input.txt");

        assert_eq!(day.part1(), Ok(1371));
    }

    #[test]
    fn test_part2() {
        let day = Matchsticks::load("data/day8_input.txt");

        assert_eq!(day.part2(), Ok(2117));
    }
}
