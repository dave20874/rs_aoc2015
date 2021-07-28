use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

pub struct Day13 {
    happy_units: HashMap<(String, String), i32>,
    people: Vec<String>,
}

impl Day13 {
    pub fn load(filename: &str) -> Day13 {
        let mut map: HashMap<(String, String), i32> = HashMap::new();
        let mut people: Vec<String> = Vec::new();

        lazy_static! {
            static ref STATEMENT_RE: Regex = Regex::new("(.+) would (.+) ([0-9]+) happiness units by sitting next to (.+)\\.").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            match STATEMENT_RE.captures(l) {
                Some(cap) => {
                    let mut score: i32 = cap[3].parse().unwrap();
                    if &cap[2] == "lose" {
                        score = -score;
                    }
                    map.insert( (cap[1].to_string(), cap[4].to_string()), score);

                    let subject: String = cap[1].to_string();
                    if !people.contains(&subject) {
                        people.push(subject);
                    }
                }
                _ => {}
            }
        }

        // Add zero relationships from everyone to me and me to everyone.
        let me = "me";
        for person in &people {
            map.insert((me.to_string(), person.to_string()), 0);
            map.insert( (person.to_string(), me.to_string()), 0);
        }

        Day13 { happy_units: map, people: people }
    }

    fn get_optimal(&self, add_self: bool) -> i32 {
        let mut people = self.people.clone();
        if add_self {
            people.push("me".to_string());
        }

        let num_people = people.len();

        let mut best: i32 = 0;
        let mut best_set: bool = false;
        for seating in people.iter().permutations(num_people) {
            let mut score = 0;
            for i in 0..num_people {
                let subject = seating.get(i).unwrap();
                // print!("{}, ", subject);
                let left = (i + num_people - 1) % num_people;
                let left_person = seating.get(left).unwrap();
                let right = (i + 1) % num_people;
                let right_person = seating.get(right).unwrap();
                score += self.happy_units.get( &(subject.to_string(), left_person.to_string()) ).unwrap();
                score += self.happy_units.get(&(subject.to_string(), right_person.to_string()) ).unwrap();
            }
            // println!();

            if !best_set || (score > best) {
                best = score;
                // println!("  New best: {}.", best);
                best_set = true;
            }
        }

        best
    }
}

impl super::Day for Day13 {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.get_optimal(false) as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.get_optimal(true) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day13::load("data/day13_example1.txt");
        assert_eq!(d.happy_units.len(), 12);
        assert_eq!(d.people.len(), 4);
    }

    #[test]
    fn test_optimal() {
        let d = Day13::load("data/day13_example1.txt");
        assert_eq!(d.get_optimal(false), 330);
    }

    #[test]
    fn test_part1() {
        let d = Day13::load("data/day13_input.txt");
        assert_ne!(d.part1(), Ok(65));  // It's not 65!
        assert_eq!(d.part1(), Ok(733));
    }

    #[test]
    fn test_part2() {
        let d = Day13::load("data/day13_input.txt");
        assert_eq!(d.part2(), Ok(725));
    }
}