use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use regex::Matches;
use pathfinding::astar;

type Compound = Vec<u32>;

pub struct Day19 {
    // Elements are assigned integer representations.
    // A vector of u32 represents a compound.

    // Mapping of element name to int
    elt_to_num: HashMap<String, u32>,
    /// num_to_elt: Vec<String>,

    // Each element can map to a new compound in multiple ways.
    rules: HashMap<u32, Vec<Compound>>,
    medicine: Compound,
}

impl Day19 {
    fn register_elt<'a>(name: &str, elt_to_num: &mut HashMap<String, u32>) {
        if !elt_to_num.contains_key(name) {
            let new_num: u32 = elt_to_num.len() as u32;
            elt_to_num.insert(name.to_string(), new_num);
        }
    }

    pub fn load(filename: &str) -> Day19 {
        // These are the components we'll put in the Day19 struct
        let mut elt_to_num: HashMap<String, u32> = HashMap::new();
        /// let mut num_to_elt: Vec<String> = Vec::new();
        let mut rules: HashMap<u32, Vec<Compound>> = HashMap::new();
        let mut medicine: Vec<u32> = Vec::new();

        // For file processing
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut in_rules = true;
        lazy_static! {
            // REPL_RE[1] -> element
            // REPL_RE[2] -> compound
            static ref REPL_RE: Regex = Regex::new("([eA-Z][a-z]?) => (([A-Z][a-z]?)+)").unwrap();
            static ref ELEMENT_RE: Regex = Regex::new("([A-Z][a-z]?)").unwrap();
        }


        // TODO-DW : Add
        let mut elt_rules: Vec<Compound> = Vec::new();
        let mut last_elt: Option<u32> = None;

        for l in reader.lines() {
            // if blank line, negate rules
            let l = l.unwrap();
            if l.is_empty() {
                // switch from cataloging rules to recording medicine compound
                in_rules = false;

                // Close rules by adding the last elt_rules under construction to rules
                match last_elt {
                    Some(elt) => {
                        rules.insert(elt, elt_rules);
                        elt_rules = Vec::new();
                    },
                    _ => {}
                }
            }
            else if in_rules {
                // interpret transformation rule
                match REPL_RE.captures(&l) {
                    Some(cap) => {
                        let elt: &str = &cap[1];
                        let compound: &str = &cap[2];

                        // make sure all elements are registered
                        Day19::register_elt(elt, &mut elt_to_num);
                        for compound_elt in ELEMENT_RE.captures_iter(compound) {
                            Day19::register_elt(&compound_elt[0], &mut elt_to_num);
                        }

                        // convert elt to a number
                        let mut elt_num = elt_to_num[elt];

                        // If elt_num differs from last one processed, close out the previous
                        // elt_rules.
                        if last_elt != Some(elt_num) {
                            match last_elt {
                                Some(e) => {
                                    rules.insert(e, elt_rules);
                                    elt_rules = Vec::new();
                                },
                                _ => {}
                            }
                            last_elt = Some(elt_num);
                        }

                        // Get compound as vec
                        let mut compound_vec: Vec<u32> = Vec::new();
                        for elt_cap in ELEMENT_RE.captures_iter(compound) {
                            compound_vec.push(elt_to_num[&elt_cap[0]]);
                        }

                        elt_rules.push(compound_vec);
                    }
                    _ => {
                        // Ignore lines that don't match
                        println!("Uh, why no match? {}", l);
                    }
                }
            }
            else {
                // interpret medicine compound
                for cap in ELEMENT_RE.captures_iter(&l) {
                    let elt: &str = &cap[1];

                    // ensure this elt is registered
                    Day19::register_elt(&elt.to_string(), &mut elt_to_num);

                    // convert elt to a number
                    let mut elt_num = elt_to_num[elt];

                    medicine.push(elt_num);
                }
            }
        }

        Day19 {elt_to_num: elt_to_num, rules, medicine }
    }

    fn generate(&self, start: &Compound, generated: &mut HashMap<Vec<u32>, bool>, len_limit: usize) {
        // For N in len(medicine)
        for n in 0..start.len() {
            let key_elt = start[n];
            if self.rules.contains_key(&key_elt) {
                for replacement in &self.rules[&key_elt] {
                    let mut new_compound: Vec<u32> = Vec::new();
                    for i in 0..start.len() {
                        if i == n {
                            // replace key element with replacement compound
                            for elt in replacement {
                                new_compound.push(*elt);
                            }
                        }
                        else {
                            // keep element
                            new_compound.push(start[i]);
                        }
                    }

                    // Trim the search tree by excluding compounds that get too long.
                    if (len_limit == 0) || (new_compound.len() <= len_limit) {
                        generated.insert(new_compound, true);
                    }
                }
            }
        }
        //   For rule in rules for elt at medicine[N]
        //     Generate variations of medicine with 0-N-1, the same, replace elt with rule, N+1-end the same.
        //     Add each variation to hashmap as a key.  Value is true.
        // Done.
    }

    // Produce an iterator that yields (Compound, cost) where cost is always 1.
    // (cost is 1 because we are measuring how many steps to get from medicine back to 'e')
    fn precursors(&self, c: Compound) -> Vec<(Compound, u32)> {
        let mut v: Vec<(Compound, u32)> = Vec::new();

        // Loop over positions in compound
        for n in 0..c.len() {
            // Loop over elements with reactions
            for (precursor_elt, compounds) in self.rules.iter() {
                for sub_compound in compounds {
                    // If the elements at c match sub_compound, generate a precursor compound
                    if (c.len() >= n+sub_compound.len()) &&
                        (c[n..n+sub_compound.len()] == sub_compound) {
                        // generate a precursor compound where precursor_elt is at
                        // position n instead of sub_compound
                        let mut precursor: Compound = Vec::new();
                        for i in 0..n {
                            precursor.push(c[i]);
                        }
                        precursor.push(*precursor_elt);
                        for i in n+sub_compound.len()..c.len() {
                            precursor.push(c[i]);
                        }

                        // Add this precursor to the return vector, v, with a cost of 1.
                        v.push( (precursor, 1) );
                    }
                }
            }
        }

        // return the vector v
        v
    }
}

impl super::Day for Day19 {
    fn part1(&mut self) -> Result<i64, &str> {
        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();

        self.generate(&self.medicine, &mut generated, 0);

        Ok(generated.len() as i64)
    }

    fn part2(&mut self) -> Result<i64, &str> {
        let finding = astar(&self.medicine, self.precursors(c), |p| 1, |p| *p == self.elt_to_num['e']);

        match finding {
            Some((_, cost)) => Ok(cost as i64),
            None => None
        }
    }

    /*
    fn part2(&mut self) -> Result<i64, &str> {
        let mut a: HashMap<Vec<u32>, bool> = HashMap::new();
        let mut b: HashMap<Vec<u32>, bool> = HashMap::new();
        let mut steps = 0;

        let mut current = &mut a;
        let mut next = &mut b;

        let e = vec![self.elt_to_num["e"]];
        // println!("e is {:?}", e);
        current.insert(e, true);
        while !current.contains_key(&self.medicine) {
            for (compound, _) in current.iter() {
                self.generate(compound, next, self.medicine.len());
            }
            steps += 1;
            println!("Steps increased to {}", steps);
            let temp = current;
            current = next;
            next = temp;
            next.clear();
            // println!("Gen {}: {:?}", steps, current.keys());
        }

        Ok(steps as i64)
    }
     */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day19::load("data/day19_example1.txt");
        assert_eq!(d.rules.len(), 3);
        assert_eq!(d.medicine, vec!{0, 1, 0});

        let d = Day19::load("data/day19_example2.txt");
        assert_eq!(d.rules.len(), 3);
        assert_eq!(d.medicine, vec!{0, 1, 0, 1, 0, 1});
    }

    #[test]
    fn test_generate() {
        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();
        let d = Day19::load("data/day19_example1.txt");
        d.generate(&d.medicine, &mut generated, 0);
        assert_eq!(generated.len(), 4);

        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();
        let d = Day19::load("data/day19_example2.txt");
        d.generate(&d.medicine, &mut generated, 0);
        assert_eq!(generated.len(), 7);
    }

    #[test]
    fn test_part1() {
        let mut d = Day19::load("data/day19_input.txt");
        assert_eq!(d.part1(), Ok(509));
    }

    #[test]
    fn test_part2() {
        let mut d = Day19::load("data/day19_example1.txt");
        assert_eq!(d.part2(), Ok(3));

        let mut d = Day19::load("data/day19_example2.txt");
        assert_eq!(d.part2(), Ok(6));

        let mut d = Day19::load("data/day19_input.txt");
        assert_eq!(d.part2(), Ok(99));
    }
}