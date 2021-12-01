use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Reverse;
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;

type Compound = Vec<u32>;

pub struct Day19 {
    // Elements are assigned integer representations.
    // A vector of u32 represents a compound.

    // Mapping of element name to int
    // elt_to_num: HashMap<String, u32>,
    e: u32,

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
        let mut rules: HashMap<u32, Vec<Compound>> = HashMap::new();
        let mut medicine: Vec<u32> = Vec::new();

        // For file processing
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut in_rules = true;
        lazy_static! {
            // REPL_RE[1] -> element
            // REPL_RE[2] -> compound
            static ref REPL_RE: Regex = Regex::new("([eA-Z][a-z]?) => (([A-Z][a-z]?)+)").unwrap();
            static ref ELEMENT_RE: Regex = Regex::new("([A-Z][a-z]?)").unwrap();
        }

        let mut elt_rules: Vec<Compound> = Vec::new();
        let mut last_elt: Option<u32> = None;

        // register a fake element, "." to serve as a start/end marker.
        Day19::register_elt(".", &mut elt_to_num);

        // register 'e' as element 1.
        Day19::register_elt("e", &mut elt_to_num);

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
                        let elt_num = elt_to_num[elt];

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
                    Day19::register_elt(&elt.to_string(), &mut elt_to_num, /* &mut num_to_elt */);

                    // convert elt to a number
                    let elt_num = elt_to_num[elt];

                    medicine.push(elt_num);
                }
            }
        }

        let e = elt_to_num["e"];

        Day19 {e, rules, medicine }
    }

    fn generate(&self, start: &Compound, generated: &mut HashMap<Vec<u32>, bool>) {
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

                    generated.insert(new_compound, true);
                }
            }
        }
    }

    fn is_valid_compound(&self, c: &Compound, reachable: &HashMap<Compound, bool>) -> bool {
        // compounds may not contain 'e' unless they are just one element long.
        if (c.len() > 1) && (c.contains(&self.e)) {
            // The compound has an 'e' but it's not just 'e'
            // println!("Rejected premature e: {:?}", c);
            return false;
        }

        // check all subsequences of 5 or more elements
        if c.len() >= 5 {
            for n in 0..c.len()-5 {
                if !reachable.contains_key(&c[n..n+5]) {
                    // The compound has a subsequence that's unreachable.
                    return false;
                }
            }
        }

        // println!("Accepted.");
        return true;
    }

    // Produce an iterator that yields (Compound, cost) where cost is always 1.
    // (cost is 1 because we are measuring how many steps to get from medicine back to 'e')
    fn precursors(&self, c: &Compound) -> Vec<Box<Compound>> {
        let mut v: Vec<Box<Compound>> = Vec::new();

        // println!("Processing: {}, {:?}", c.len(), c);

        // Loop over positions in compound
        for n in 0..c.len() {
            // Loop over elements with reactions
            for (precursor_elt, compounds) in self.rules.iter() {
                for sub_compound in compounds {
                    // If the elements at c match sub_compound, generate a precursor compound
                    if (c.len() >= n+sub_compound.len()) &&
                        (c[n..n+sub_compound.len()] == sub_compound[0..sub_compound.len()]) {
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

                        v.push(Box::new(precursor));
                    }
                }
            }
        }

        if v.len() == 0 {
            // println!("No way forward from {} {:?}", c.len(), c);
        }
        else {
            // println!("Best precursor: {}", fwd);
        }

        // return the vector v
        v
    }

    fn reachables(&self, max: usize) -> HashMap<Compound, bool> {
        // construct set of "reachable" subsequences up to 3 elements
        let mut reachable: HashMap<Compound, bool> = HashMap::new();
        let mut to_work: Vec<Compound> = Vec::new();
        let mut new_compounds: HashMap<Compound, bool> = HashMap::new();
        let initial_compound: Vec<u32> = vec![self.e];
        to_work.push(initial_compound);
        while !to_work.is_empty() {
            let start = to_work.pop().unwrap();
            if !reachable.contains_key(&start) {
                reachable.insert(start.to_vec(), true);

                self.generate(&start, &mut new_compounds);
                for compound in new_compounds.keys() {
                    // generate all sub-compounds from this compound and add them to reachable.
                    for n in 1..=max {
                        for start in 0..compound.len() {
                            if start + n <= compound.len() {
                                let mut new_seq: Compound = Vec::new();
                                for i in 0..n {
                                    new_seq.push(compound[start + i]);
                                }
                                to_work.push(new_seq);
                            }
                        }
                    }
                }
                new_compounds.clear();
            }
        }

        // println!("Identified {} reachable subsequences of len {}.", reachable.len(), max);
        // println!("{:?}", reachable);

        reachable
    }

    fn search(&self) -> Option<u32> {
        let mut pq: PriorityQueue<(Box<Compound>, u32), Reverse<usize>> = PriorityQueue::new();

        // generate a set of all subsequences of 5 elements or fewer, that can be generated
        // under these rules.  This will be used to disqualify candidates as we work backwards
        // from the medicine molecule to 'e'.
        let reachable5 = self.reachables(5);

        // push the medicine molecule in 0 steps onto the priority queue.
        // We use the molecule length as priority in a (Reverse) priority queue
        // so we work shorter molecules first.
        let mut best = 1000;
        pq.push((Box::new(self.medicine.to_vec()), 0), Reverse(self.medicine.len()));
        while !pq.is_empty() {
            // pop the next best thing
            let (item, _priority) = pq.pop().unwrap();
            let (compound, steps) = item;
            best = std::cmp::min(best, compound.len());
            // println!("Best {} Working on {}, {}", best, compound.len(), self.compound_name(&compound));

            // If it's the origin, e, we're done.
            if (compound.len() == 1) && (compound[0] == self.e) {
                return Some(steps);
            }

            // generate all possible precursors and
            // push each one onto the priority queue
            for precursor in self.precursors(&compound) {
                if self.is_valid_compound(&precursor, &reachable5) {
                    let priority = precursor.len();
                    pq.push((precursor, steps+1), Reverse(priority));
                }
            }
            // println!("  {} precursors.", count);

            drop(compound);
        }

        return None

    }
}

impl super::Day for Day19 {
    fn part1(&mut self) -> Result<i64, &str> {
        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();

        self.generate(&self.medicine, &mut generated);

        Ok(generated.len() as i64)
    }

    fn part2(&mut self) -> Result<i64, &str> {
        match self.search() {
            Some(steps) => Ok(steps as i64),
            None => Err("No Solution")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day19::load("data/day19_example1.txt");
        assert_eq!(d.rules.len(), 3);
        assert_eq!(d.medicine, vec!{2, 3, 2});

        let d = Day19::load("data/day19_example2.txt");
        assert_eq!(d.rules.len(), 3);
        assert_eq!(d.medicine, vec!{2, 3, 2, 3, 2, 3});
    }

    #[test]
    fn test_generate() {
        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();
        let d = Day19::load("data/day19_example1.txt");
        d.generate(&d.medicine, &mut generated);
        assert_eq!(generated.len(), 4);

        let mut generated: HashMap<Vec<u32>, bool> = HashMap::new();
        let d = Day19::load("data/day19_example2.txt");
        d.generate(&d.medicine, &mut generated);
        assert_eq!(generated.len(), 7);
    }

    #[test]
    fn test_part1() {
        let mut d = Day19::load("data/day19_input.txt");
        assert_eq!(d.part1(), Ok(509));
    }

    #[test]
    fn test_reachable() {
        let d = Day19::load("data/day19_example1.txt");
        let reachables = d.reachables(3);
        assert_eq!(reachables.len(), 15);

        let d = Day19::load("data/day19_example2.txt");
        let reachables = d.reachables(3);
        assert_eq!(reachables.len(), 15);

        let d = Day19::load("data/day19_input.txt");
        let reachables = d.reachables(3);
        assert_eq!(reachables.len(), 395);
        let reachables = d.reachables(5);
        assert_eq!(reachables.len(), 6455);
    }

    #[test]
    fn test_part2() {
        let mut d = Day19::load("data/day19_example1.txt");
        assert_eq!(d.part2(), Ok(3));

        let mut d = Day19::load("data/day19_example2.txt");
        assert_eq!(d.part2(), Ok(6));

        let mut d = Day19::load("data/day19_input.txt");
        assert_eq!(d.part2(), Ok(195));
    }
}