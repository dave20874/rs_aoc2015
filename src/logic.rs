use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    UNARY, AND, OR, NOT, LSHIFT, RSHIFT,
}

// A Gate structure takes one or two inputs, performs an operation and produces
// an output.  Operands are represented as strings which may be numeric literals
// or alphabetic names of input nodes.
struct Gate {
    operation: Operation,
    operand1: String,
    operand2: String,
    _output: String,
}

impl Gate {
    fn eval(&self, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
        // println!("Evaluating {:?}: {}, {} -> {}",
        //          self.operation, self.operand1, self.operand2, self._output);

        match self.operation {
            Operation::UNARY => {
                circuit.eval_cached(&self.operand1, cache)
            }
            Operation::AND => {
                circuit.eval_cached(&self.operand1, cache) &
                    circuit.eval_cached(&self.operand2, cache)
            }
            Operation::OR => {
                circuit.eval_cached(&self.operand1, cache) |
                    circuit.eval_cached(&self.operand2, cache)
            }
            Operation::NOT => {
                !circuit.eval_cached(&self.operand1, cache)
            }
            Operation::LSHIFT => {
                circuit.eval_cached(&self.operand1, cache) <<
                    circuit.eval_cached(&self.operand2, cache)
            }
            Operation::RSHIFT => {
                circuit.eval_cached(&self.operand1, cache) >>
                    circuit.eval_cached(&self.operand2, cache)
            }
        }
    }
}

// Represents a circuit, as read from the daily input file.
pub struct Circuit {
    // Output nodes map to the Gate structures that produces them.
    gates: HashMap<String, Gate>,
}

impl Circuit {
    pub fn load(filename: &str) -> Circuit {
        let mut gates: HashMap<String, Gate> = HashMap::new();

        lazy_static! {
            static ref GATE_RE: Regex = Regex::new("(.*) -> ([a-z]+)").unwrap();
            static ref UNARY_RE: Regex = Regex::new("([0-9]+|[a-z]+)").unwrap();
            static ref AND_RE: Regex = Regex::new("([0-9]+|[a-z]+) AND ([0-9]+|[a-z]+)").unwrap();
            static ref OR_RE: Regex = Regex::new("([0-9]+|[a-z]+) OR ([0-9]+|[a-z]+)").unwrap();
            static ref NOT_RE: Regex = Regex::new("NOT ([0-9]+|[a-z]+)").unwrap();
            static ref LSHIFT_RE: Regex = Regex::new("([0-9]+|[a-z]+) LSHIFT ([0-9]+|[a-z]+)").unwrap();
            static ref RSHIFT_RE: Regex = Regex::new("([0-9]+|[a-z]+) RSHIFT ([0-9]+|[a-z]+)").unwrap();
        }
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            match GATE_RE.captures(l) {
                Some(cap) => {
                    let expr= &cap[1];
                    let output = cap[2].to_string();
                    let mut operand1: String = String::from("");
                    let mut operand2: String = String::from("");
                    let mut operation = Operation::UNARY;

                    // Check for Unary operation
                    match UNARY_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::UNARY;
                            operand1 = expr_cap[1].to_string();
                        }
                        None => ()
                    }

                    // Check for AND operation
                    match AND_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::AND;
                            operand1 = expr_cap[1].to_string();
                            operand2 = expr_cap[2].to_string();
                        }
                        None => ()
                    }

                    // Check for OR operation
                    match OR_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::OR;
                            operand1 = expr_cap[1].to_string();
                            operand2 = expr_cap[2].to_string();
                        }
                        None => ()
                    }

                    // Check for NOT operation
                    match NOT_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::NOT;
                            operand1 = expr_cap[1].to_string();
                        }
                        None => ()
                    }

                    // Check for LSHIFT operation
                    match LSHIFT_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::LSHIFT;
                            operand1 = expr_cap[1].to_string();
                            operand2 = expr_cap[2].to_string();
                        }
                        None => ()
                    }

                    // Check for RSHIFT operation
                    match RSHIFT_RE.captures(expr) {
                        Some(expr_cap) => {
                            operation = Operation::RSHIFT;
                            operand1 = expr_cap[1].to_string();
                            operand2 = expr_cap[2].to_string();
                        }
                        None => ()
                    }

                    // println!("Storing {:?}: {}, {} -> {}",
                    //          operation, operand1, operand2, output);

                    gates.insert(output.to_string(),
                                 Gate { operation: operation,
                                     operand1: operand1,
                                     operand2: operand2,
                                     _output: output } );
                }
                _ => {}
            }
        }

        Circuit { gates: gates }
    }

    fn eval_cached(&self, node: &str, cache: &mut HashMap<String, u16>) -> u16 {
        match cache.get(node) {
            Some(v) => *v,
            None => {
                let newval = match node.parse::<u16>() {
                    Ok(value) => {
                        // println!("Evaluated constant: {}", value);
                        value
                    },
                    Err(_) => {
                        // Find the gate with this node name as output and evaluate it.
                        match self.gates.get(node) {
                            Some(gate) => gate.eval(self, cache),
                            None => panic!("Could not get gate."),
                        }
                    }
                };

                cache.insert(node.to_string(), newval);
                newval
            }
        }
    }

    fn eval(&self, node: &str) -> u16 {
        let mut cache: HashMap<String, u16> = HashMap::new();

        self.eval_cached(node, &mut cache)
    }
}

impl super::Day for Circuit {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.eval("a") as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        // "Take the signal you got on wire a..."
        let initial_a = self.eval("a");

        // "Override wire b to that signal..."
        let mut cache: HashMap<String, u16> = HashMap::new();
        cache.insert("b".to_string(), initial_a);

        // "What new signal is ultimately provided to wire a?"
        let ultimate_a = self.eval_cached("a", &mut cache);

        return Ok(ultimate_a as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let ckt = Circuit::load("data/day7_example1.txt");

        assert_eq!(ckt.gates.len(), 8);
    }

    #[test]
    fn test_example1() {
        let ckt = Circuit::load("data/day7_example1.txt");

        let cases = [
            ("99", 99),
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        for (node, expected) in cases {
            assert_eq!(ckt.eval(node), expected);
        }
    }

    #[test]
    fn test_part1() {
        let day = Circuit::load("data/day7_input.txt");
        assert_eq!(day.part1(), Ok(46065));
    }

    #[test]
    fn test_part2() {
        let day = Circuit::load("data/day7_input.txt");
        assert_eq!(day.part2(), Ok(14134));
    }
}
