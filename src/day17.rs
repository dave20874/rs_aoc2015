use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day17 {
    containers: Vec<usize>,
}

impl Day17 {
    pub fn load(filename: &str) -> Day17 {
        let mut v: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let val = l.parse::<usize>().unwrap();
            v.push(val);
        }
        v.sort();
        v.reverse();

        Day17 { containers: v }
    }

    // Counts all combos that fit the total.
    fn combos_recurse(&self, i: usize, so_far: usize,
                      containers_used: usize, containers_allowed: usize,
                      total: usize) -> usize {
        if (containers_allowed > 0) && (containers_used > containers_allowed) {
            // used to many containers, quit
            return 0;
        }
        if so_far == total {
            // Hey hey!  It's a fit.  Search no further
            // println!("  Match!");
            return 1;
        }
        else if so_far > total {
            // We're over capacity, stop this line of search
            // println!("  Over.");
            return 0;
        }
        else if i >= self.containers.len() {
            // We've gone past the end!
            // println!("  Too far.");
            return 0;
        }
        else {
            // Try further combos with and without including the current index.
            let next_size = self.containers.get(i).unwrap();
            // println!("With {}", next_size);
            let with = self.combos_recurse(i+1, so_far+next_size, containers_used+1, containers_allowed, total);
            // println!("Without {}", next_size);
            let without = self.combos_recurse(i+1, so_far, containers_used, containers_allowed, total);

            return with + without;
        }
    }

    fn combos(&self, total: usize, min_containers: bool) -> usize {
        if !min_containers {
            self.combos_recurse(0, 0, 0, 0, total)
        }
        else {
            let mut containers_allowed = 0;
            let mut num_combos = 0;
            while num_combos == 0 {
                containers_allowed += 1;
                num_combos = self.combos_recurse(0, 0, 0, containers_allowed, total);
            }

            num_combos
        }

    }
}

impl super::Day for Day17 {
    fn part1(&mut self) -> Result<i64, &str> {
        Ok(self.combos(150, false) as i64)
    }

    fn part2(&mut self) -> Result<i64, &str> {
        Ok(self.combos(150, true) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.containers.len(), 5);

        let d = Day17::load("data/day17_input.txt");
        assert_eq!(d.containers.len(), 20);
    }

    #[test]
    fn test_combos_ex1() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.combos(25, false), 4);
    }

    #[test]
    fn test_part1() {
        let mut d = Day17::load("data/day17_input.txt");
        assert_eq!(d.part1(), Ok(1638));
    }

    #[test]
    fn test_min_combos_ex1() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.combos(25, true), 3);
    }

    #[test]
    fn test_part2() {
        let mut d = Day17::load("data/day17_input.txt");
        assert_eq!(d.part2(), Ok(17));
    }
}