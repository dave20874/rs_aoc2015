use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day14 {
    reindeer: HashMap<String, (u32, u32, u32)>,  // Name -> speed, endurance, rest
}

impl Day14 {
    pub fn load(filename: &str) -> Day14 {
        let mut reindeer: HashMap<String, (u32, u32, u32)> = HashMap::new();

        lazy_static! {
            static ref STATEMENT_RE: Regex = Regex::new("(.+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds\\.").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            match STATEMENT_RE.captures(l) {
                Some(cap) => {
                    let name = &cap[1];
                    let speed: u32 = cap[2].parse().unwrap();
                    let endurance: u32 = cap[3].parse().unwrap();
                    let rest: u32 = cap[4].parse().unwrap();

                    reindeer.insert(name.to_string(), (speed, endurance, rest));
                }
                _ => {}
            }
        }

        Day14 { reindeer: reindeer }
    }

    fn distance(speed: u32, endurance: u32, rest: u32, time: u32) -> u32 {
        let full_cycles = time / (endurance+rest);
        let partial_cycle = time % (endurance+rest);
        let extra_flight_time = if partial_cycle > endurance { endurance } else { partial_cycle };
        let flight_time = full_cycles * endurance + extra_flight_time;
        let distance = flight_time * speed;

        distance
    }

    fn reindeer_distance(&self, name: &str, time: u32) -> u32 {
        let (speed, endurance, rest) = self.reindeer.get(name).unwrap();

        Day14::distance(*speed, *endurance, *rest, time)
    }

    fn max_distance(&self, time: u32) -> (String, u32) {
        let mut max: u32 = 0;
        let mut winner: &str = "";

        for reindeer in self.reindeer.keys() {
            let d = self.reindeer_distance(reindeer, time);
            if d > max {
                max = d;
                winner = reindeer;
            }
        }

        (winner.to_string(), max)
    }

    fn max_points(&self, time: u32) -> (String, u32) {
        // create points map and init to zero for all reindeer
        let mut points: HashMap<String, u32> = HashMap::new();
        for reindeer in self.reindeer.keys() {
            points.insert(reindeer.to_string(), 0);
        }

        // Get the winner after 1, 2, etc seconds and update points
        for t in 1..=time {
            let (_leader, dist) = self.max_distance(t);
            for deer in self.reindeer.keys() {
                if self.reindeer_distance(deer, t) == dist {
                    // This deer gets the points (possibly in a tie with others.)
                    let entry = points.get_mut(deer).unwrap();
                    *entry += 1;
                }
            }
        }

        // Find the reindeer with the most points and return that value
        let mut max: u32 = 0;
        let mut winner= "";
        for reindeer in points.keys() {
            let p = *points.get(reindeer).unwrap();
            if p > max {
                max = p;
                winner = reindeer;
            }
        }

        (winner.to_string(), max)
    }
}

impl super::Day for Day14 {
    fn part1(&mut self) -> Result<i64, &str> {
        let (_winner, dist) = self.max_distance(2503);
        return Ok(dist as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        let (_winner, dist) = self.max_points(2503);
        return Ok(dist as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day14::load("data/day14_input.txt");
        assert_eq!(d.reindeer.len(), 9);
        let d = Day14::load("data/day14_example1.txt");
        assert_eq!(d.reindeer.len(), 2);
    }

    #[test]
    fn test_distance() {
        assert_eq!(Day14::distance(14, 10, 127, 1000), 1120);
        assert_eq!(Day14::distance(16, 11, 162, 1000), 1056);
    }

    #[test]
    fn test_reindeeer_distance() {
        let d = Day14::load("data/day14_example1.txt");
        assert_eq!(d.reindeer_distance("Comet", 1000), 1120);
        assert_eq!(d.reindeer_distance("Dancer", 1000), 1056);
    }

    #[test]
    fn test_max_distance() {
        let d = Day14::load("data/day14_example1.txt");
        let (_winner, max) = d.max_distance(1000);
        assert_eq!(max, 1120);
    }

    #[test]
    fn test_max_points() {
        let d = Day14::load("data/day14_example1.txt");
        let (_winner, points) = d.max_points(1000);
        assert_eq!(points, 689);
    }

    #[test]
    fn test_part1() {
        let mut d = Day14::load("data/day14_input.txt");
        assert_eq!(d.part1(), Ok(2696));
    }

    #[test]
    fn test_part2() {
        let mut d = Day14::load("data/day14_input.txt");
        assert_eq!(d.part2(), Ok(1084));
    }
}