use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

type Characteristics = HashMap<String, usize>;

pub struct Day16 {
    aunts: Vec<Characteristics>,
}

impl Day16 {
    pub fn load(filename: &str) -> Day16 {
        let mut aunts: Vec<Characteristics> = Vec::new();

        lazy_static! {
            static ref AUNT_RE: Regex = Regex::new("Sue [0-9]+: ([a-z]+): ([0-9]+), ([a-z]+): ([0-9]+), ([a-z]+): ([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            match AUNT_RE.captures(l) {
                Some(cap) => {
                    let mut aunt: Characteristics = HashMap::new();
                    aunt.insert(cap[1].to_string(), cap[2].parse::<usize>().unwrap());
                    aunt.insert(cap[3].to_string(), cap[4].parse::<usize>().unwrap());
                    aunt.insert(cap[5].to_string(), cap[6].parse::<usize>().unwrap());
                    aunts.push(aunt);
                }
                _ => {}
            }
        }

        Day16 {aunts: aunts}
    }

    fn find_aunt(&self, part2: bool) -> Option<usize> {
        let mut mfcsam: Characteristics = HashMap::new();
        mfcsam.insert(String::from("children"), 3);
        mfcsam.insert(String::from("cats"), 7);
        mfcsam.insert(String::from("samoyeds"), 2);
        mfcsam.insert(String::from("pomeranians"), 3);
        mfcsam.insert(String::from("akitas"), 0);
        mfcsam.insert(String::from("vizslas"), 0);
        mfcsam.insert(String::from("goldfish"), 5);
        mfcsam.insert(String::from("trees"), 3);
        mfcsam.insert(String::from("cars"), 2);
        mfcsam.insert(String::from("perfumes"), 1);

        'aunts: for (n, aunt) in self.aunts.iter().enumerate() {
            for (k, v) in &mfcsam {
                if aunt.contains_key(k) {
                    if part2 {
                        // part 2 matching logic.
                        if (k == "cats") || (k == "trees") {
                            if aunt.get(k).unwrap() <= v {
                                // mismatch: there must be > this many
                                continue 'aunts;
                            }
                        } else if (k == "pomeranians") || (k == "goldfish") {
                            if aunt.get(k).unwrap() >= v {
                                // mismatch: there must be < this many
                                continue 'aunts;
                            }
                        } else if aunt.get(k).unwrap() != v {
                            // mismatched on exact match for characteristic.
                            continue 'aunts;
                        }
                    }
                    else {
                        // part1 matching logic
                        if aunt.get(k).unwrap() != v {
                            // mismatched on exact match for characteristic.
                            continue 'aunts;
                        }
                    }
                }
            }

            // found no mismatches, return this aunt
            // note: +1 to convert from 0-based enumeration to 1-based aunt numbering
            return Some(n+1);
        }
        return None;
    }


}

impl super::Day for Day16 {
    fn part1(&mut self) -> Result<i64, &str> {
        match self.find_aunt(false) {
            Some(n) => Ok(n as i64),
            None => Err("No solution"),
        }
    }

    fn part2(&mut self) -> Result<i64, &str> {
        match self.find_aunt(true) {
            Some(n) => Ok(n as i64),
            None => Err("No solution"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day16::load("data/day16_input.txt");
        assert_eq!(d.aunts.len(), 500);
    }

    #[test]
    fn test_part1() {
        let mut d = Day16::load("data/day16_input.txt");
        assert_eq!(d.part1(), Ok(213));
    }

    #[test]
    fn test_part2() {
        let mut d = Day16::load("data/day16_input.txt");
        assert_eq!(d.part2(), Ok(323));
    }
}


