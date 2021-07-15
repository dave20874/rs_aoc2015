use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Presents {
    dims: Vec<(u32, u32, u32)>,
}

impl Presents {
    pub fn load(filename: &str) -> Presents {
        lazy_static! {
            static ref LWH_RE: Regex = Regex::new("([0-9]+)x([0-9]+)x([0-9]+)").unwrap();
        }
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut dims = Vec::new();

        for line in reader.lines() {
            let l = &line.unwrap();
            match LWH_RE.captures(l) {
                Some(cap) => {
                    // We have a match for a mask instruction
                    dims.push((cap[1].parse().unwrap(),
                               cap[2].parse().unwrap(),
                               cap[3].parse().unwrap()));
                }
                _ => {}
            }
        }

        return Presents { dims: dims }
    }

    pub fn paper_required(dims: &(u32, u32, u32)) -> u32 {
        let a1 = dims.0*dims.1;
        let a2 = dims.1*dims.2;
        let a3 = dims.2*dims.0;

        let mut min_area = a1;
        if a2 < min_area { min_area = a2; }
        if a3 < min_area { min_area = a3; }

        return 2*a1 + 2*a2 + 2*a3 + min_area;
    }

    pub fn ribbon_required(dims: &(u32, u32, u32)) -> u32 {
        let vol = dims.0*dims.1*dims.2;
        let p1 = 2*(dims.0+dims.1);
        let p2 = 2*(dims.1+dims.2);
        let p3 = 2*(dims.2+dims.0);

        let mut min_perim = p1;
        if p2 < min_perim { min_perim = p2; }
        if p3 < min_perim { min_perim = p3; }

        return min_perim + vol;
    }
}

impl super::Day for Presents {
    fn part1(&self) -> Result<i64, &str> {
        let mut accum = 0;
        for dim in &self.dims {
            accum += Presents::paper_required(&dim);
        }

        return Ok(accum as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut accum = 0;
        for dim in &self.dims {
            accum += Presents::ribbon_required(&dim);
        }

        return Ok(accum as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_paper_required() {
        let examples: Vec<((u32, u32, u32), u32)> = vec![
            ((2, 3, 4), 58),
            ((1, 1, 10), 43),
        ];

        for (dims, expected) in examples {
            let area = Presents::paper_required(&dims);
            assert_eq!(area, expected);
        }
    }

    #[test]
    fn test_ribbon_required() {
        let examples: Vec<((u32, u32, u32), u32)> = vec![
            ((2, 3, 4), 34),
            ((1, 1, 10), 14),
        ];

        for (dims, expected) in examples {
            let feet = Presents::ribbon_required(&dims);
            assert_eq!(feet, expected);
        }
    }

    #[test]
    fn test_load() {
        let day = &Presents::load("data/day2_input.txt");
        assert_eq!(day.dims.len(), 1000)
    }

    #[test]
    fn test_part1() {
        let day = &Presents::load("data/day2_input.txt");
        assert_eq!(day.part1(), Ok(1598415));
    }

    #[test]
    fn test_part2() {
        let day = &Presents::load("data/day2_input.txt");
        assert_eq!(day.part2(), Ok(3812909));
    }
}