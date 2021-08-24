use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub struct Day15 {
    ingredients: HashMap<String, Properties>,
}

impl Day15 {
    pub fn load(filename: &str) -> Day15 {
        let mut ingredients: HashMap<String, Properties> = HashMap::new();

        lazy_static! {
            static ref STATEMENT_RE: Regex = Regex::new("(.+): capacity ([\\-0-9]+), durability ([\\-0-9]+), flavor ([\\-0-9]+), texture ([\\-0-9]+), calories ([\\-0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            match STATEMENT_RE.captures(l) {
                Some(cap) => {
                    let name = &cap[1];
                    let prop = Properties {
                        capacity: cap[2].parse().unwrap(),
                        durability: cap[3].parse().unwrap(),
                        flavor: cap[4].parse().unwrap(),
                        texture: cap[5].parse().unwrap(),
                        calories: cap[6].parse().unwrap(),
                    };

                    ingredients.insert(name.to_string(), prop);
                }
                _ => {}
            }
        }

        Day15 { ingredients: ingredients }
    }
}

impl super::Day for Day15 {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(0 as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(0 as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day15::load("data/day15_input.txt");
        assert_eq!(d.ingredients.len(), 4);
        let d = Day15::load("data/day15_example1.txt");
        assert_eq!(d.ingredients.len(), 2);
    }

    #[test]
    fn test_cookie() {
        let d = Day15::load("data/day15_example1.txt");

        // TODO: Come up with a way to represent recipe
        // TODO: Butterscotch: 44, Cinnamon: 56

        // assert_eq!(d.score_cookie(recipe), 62842880);
    }
}
