use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

// Operations we read from the input file
enum Operation { ON, OFF, TOGGLE, }

// Represents one line of the input file.
struct Instruction {
    op: Operation,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

// Represents the problem, holds a vector of instructions.
pub struct Lights {
    instructions: Vec<Instruction>,
}

impl Lights {
    // Read the input file, store the instructions found there and return Lights object.
    pub fn load(filename: &str) -> Lights {
        let mut instructions: Vec<Instruction> = Vec::new();

        lazy_static! {
            static ref INSTR_RE: Regex = Regex::new("(.*) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        }
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            match INSTR_RE.captures(l) {
                Some(cap) => {
                    let x1: usize = cap[2].parse().unwrap();
                    let y1: usize = cap[3].parse().unwrap();
                    let x2: usize = cap[4].parse().unwrap();
                    let y2: usize = cap[5].parse().unwrap();

                    let op = match &cap[1] {
                        "turn on" => Operation::ON,
                        "turn off" => Operation::OFF,
                        "toggle" => Operation::TOGGLE,
                        _ => Operation::OFF,
                    };
                    instructions.push(Instruction {op: op, x1: x1, y1: y1, x2: x2, y2: y2})
                }
                _ => {}
            }
        }

        Lights { instructions: instructions }
    }

    // Interpret the instructions per part 1 and return the number of lights lit.
    fn run_part1(&self) -> u32 {
        let mut state = vec![vec![false; 1000]; 1000];

        // Run the instructions per part1
        for i in &self.instructions {
            match i.op {
                Operation::ON => {
                    // Turn on some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            state[x][y] = true;
                        }
                    }
                }
                Operation::OFF => {
                    // Turn off some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            state[x][y] = false;
                        }
                    }
                }
                Operation::TOGGLE => {
                    // Toggle some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            state[x][y] ^= true;
                        }
                    }
                }
            }
        }

        Lights::num_lit(state)
    }

    fn run_part2(&self) -> u32 {
        let mut state = vec![vec![0; 1000]; 1000];

        // Run the instructions per part1
        for i in &self.instructions {
            match i.op {
                Operation::ON => {
                    // Turn on some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            state[x][y] += 1;
                        }
                    }
                }
                Operation::OFF => {
                    // Turn off some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            if state[x][y] > 0 {
                                state[x][y] -= 1;
                            }
                        }
                    }
                }
                Operation::TOGGLE => {
                    // Toggle some lights
                    for x in i.x1..=i.x2 {
                        for y in i.y1..=i.y2 {
                            state[x][y] += 2;
                        }
                    }
                }
            }
        }

        Lights::brightness(state)
    }

    fn num_lit(state: Vec<Vec<bool>>) -> u32 {
        let mut count = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                if state[x][y] {
                    count += 1;
                }
            }
        }

        count
    }

    fn brightness(state: Vec<Vec<u32>>) -> u32 {
        let mut count = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                count += state[x][y];
            }
        }

        count
    }
}

impl super::Day for Lights {
    fn part1(&mut self) -> Result<i64, &str> {
        return Ok(self.run_part1() as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        return Ok(self.run_part2() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_example1() {
        let day = Lights::load("data/day6_example1.txt");
        assert_eq!(day.run_part1(), 1000000);
    }
    #[test]
    fn test_example2() {
        let day = Lights::load("data/day6_example2.txt");
        assert_eq!(day.run_part1(), 1000);
    }
    #[test]
    fn test_example3() {
        let day = Lights::load("data/day6_example3.txt");
        assert_eq!(day.run_part1(), 999996);
    }

    #[test]
    fn test_part1() {
        let mut day = Lights::load("data/day6_input.txt");
        assert_eq!(day.part1(), Ok(543903));
    }

    #[test]
    fn test_part2() {
        let mut day = Lights::load("data/day6_input.txt");
        assert_eq!(day.part2(), Ok(14687245));
    }
}