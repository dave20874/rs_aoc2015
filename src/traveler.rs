use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

pub struct Traveler {
    distance: HashMap<(String, String), u32>,
    places: Vec<String>,
}

impl Traveler {
    pub fn load(filename: &str) -> Traveler {
        let mut distance: HashMap<(String, String), u32> = HashMap::new();
        let mut places: Vec<String> = Vec::new();

        lazy_static! {
            static ref DIST_RE: Regex = Regex::new("([a-zA-Z]+) to ([a-zA-Z]+) = ([0-9]+)").unwrap();
        }
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            match DIST_RE.captures(l) {
                Some(cap) => {
                    let dist: u32 = cap[3].parse().unwrap();
                    distance.insert((cap[1].to_string(), cap[2].to_string()), dist);
                    distance.insert((cap[2].to_string(), cap[1].to_string()), dist);

                    if !places.contains(&cap[1].to_string()) {
                        places.push(cap[1].to_string());
                    }
                    if !places.contains(&cap[2].to_string()) {
                        places.push(cap[2].to_string());
                    }
                }
                _ => ()
            }
        }

        Traveler { distance: distance, places: places }
    }

    fn get_distance(&self, path: &Vec<&String>) -> u32 {
        let mut total = 0;
        for n in 0..path.len()-1 {
            // barf!
            let d = self.distance.get( &(path[n].to_string(), path[n+1].to_string())).unwrap();
            // println!("to {}, {}", self.places[n+1], d);

            total += d;
        }

        total
    }

    fn shortest_path(&self) -> u32 {
        let num_places = self.places.len();
        let mut shortest = 0;
        for path in self.places.iter().permutations(num_places) {
            let d = self.get_distance(&path);
            if (shortest == 0) || (d < shortest) {
                shortest = d;
            }
        }

        shortest
    }

    fn longest_path(&self) -> u32 {
        let num_places = self.places.len();
        let mut longest = 0;
        for path in self.places.iter().permutations(num_places) {
            let d = self.get_distance(&path);
            if d > longest {
                longest = d;
            }
        }

        longest
    }
}

impl super::Day for Traveler {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.shortest_path() as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.longest_path() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let day = Traveler::load("data/day9_input.txt");

        assert_eq!(day.distance.len(), 56);
        assert_eq!(day.places.len(), 8);
    }

    #[test]
    fn test_min_dist() {
        let day = Traveler::load("data/day9_example1.txt");
        assert_eq!(day.shortest_path(), 605);
    }

    #[test]
    fn test_part1() {
        let day = Traveler::load("data/day9_input.txt");
        assert_eq!(day.part1(), Ok(207));
    }

    #[test]
    fn test_part2() {
        let day = Traveler::load("data/day9_input.txt");
        assert_eq!(day.part2(), Ok(804));
    }
}
