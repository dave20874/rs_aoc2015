use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::Value;

pub struct Day12 {
    val: Value,
}

impl Day12 {
    pub fn load(filename: &str) -> Day12 {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut s: String = String::new();
        reader.read_line(&mut s).unwrap();
        let json_val = serde_json::from_str(&s).unwrap();

        Day12 { val: json_val }
    }

    fn new(content: &str) -> Day12 {
        let json_val = serde_json::from_str(content).unwrap();
        Day12 { val: json_val }
    }

    fn sum_helper(js: &Value, block_red: bool) -> i64 {
        let mut sum: i64 = 0;

        match js {
            Value::Number(n) => {
                sum += n.as_i64().unwrap();
            }
            Value::Object(o) => {
                for (_, sub) in o.iter() {
                    if block_red {
                        match sub {
                            Value::String(s) => {
                                if s == "red" {
                                    // Saw "red" in an object.  Ignore the whole object.
                                    sum = 0;
                                    break;
                                }
                            }
                            _ => {
                                sum += Day12::sum_helper(sub, block_red);
                            }
                        }
                    }
                    else {
                        sum += Day12::sum_helper(sub, block_red);
                    }
                }
            }
            Value::Array(a) => {
                for sub in a {
                    sum += Day12::sum_helper(sub, block_red);
                }
            }
            _ => {}
        }

        sum
    }

    fn sum_nums(&self, block_red: bool) -> i64 {
        Day12::sum_helper(&self.val, block_red)
    }
}

impl super::Day for Day12 {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.sum_nums(false) as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.sum_nums(true) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_sum() {
        let d = Day12::new("[1,2,3]");
        assert_eq!(d.sum_nums(false), 6);
        let d = Day12::new("{\"a\":2,\"b\":4}");
        assert_eq!(d.sum_nums(false), 6);
        let d = Day12::new("[[[3]]]");
        assert_eq!(d.sum_nums(false), 3);
        let d = Day12::new("{\"a\":{\"b\":4},\"c\":-1}");
        assert_eq!(d.sum_nums(false), 3);
        let d = Day12::new("{\"a\":[-1,1]}");
        assert_eq!(d.sum_nums(false), 0);
        let d = Day12::new("[-1,{\"a\":1}]");
        assert_eq!(d.sum_nums(false), 0);
        let d = Day12::new("[]");
        assert_eq!(d.sum_nums(false), 0);
        let d = Day12::new("{}");
        assert_eq!(d.sum_nums(false), 0);

        let d = Day12::new("[1,{\"c\":\"red\",\"b\":2},3]");
        assert_eq!(d.sum_nums(true), 4);
        let d = Day12::new("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}");
        assert_eq!(d.sum_nums(true), 0);
        let d = Day12::new("[1,\"red\",5]");
        assert_eq!(d.sum_nums(true), 6);
    }

    #[test]
    fn test_part1() {
        let d = Day12::load("data/day12_input.txt");
        assert_eq!(d.part1(), Ok(191164));
    }

    #[test]
    fn test_part2() {
        let d = Day12::load("data/day12_input.txt");
        assert_eq!(d.part2(), Ok(87842));
    }
}
