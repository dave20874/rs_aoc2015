use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use lazy_static::lazy_static;
use regex::Regex;

struct Ingredient {
    _name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

pub struct Day15 {
    ingredients: Vec<Ingredient>,
    all_recipes: Vec<Vec<i64>>,
}

impl Day15 {
    pub fn load(filename: &str) -> Day15 {
        let mut ingredients: Vec<Ingredient> = Vec::new();

        lazy_static! {
            static ref STATEMENT_RE: Regex = Regex::new("(.+): capacity ([\\-0-9]+), durability ([\\-0-9]+), flavor ([\\-0-9]+), texture ([\\-0-9]+), calories ([\\-0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            match STATEMENT_RE.captures(l) {
                Some(cap) => {
                    let ingredient = Ingredient {
                        _name: cap[1].to_string(),
                        capacity: cap[2].parse().unwrap(),
                        durability: cap[3].parse().unwrap(),
                        flavor: cap[4].parse().unwrap(),
                        texture: cap[5].parse().unwrap(),
                        calories: cap[6].parse().unwrap(),
                    };

                    ingredients.push(ingredient);
                }
                _ => {}
            }
        }

        let mut recipes: Vec<Vec<i64>> = vec!();
        Day15::gen_all_recipes(&mut recipes, ingredients.len(), 100);

        Day15 { ingredients: ingredients, all_recipes: recipes }
    }


    fn gen_all_recurse(recipes: &mut Vec<Vec<i64>>, next_i: usize, remaining: i64, recipe: &mut Vec<i64>) {

        if next_i == recipe.len() {
            // all ingredients allocated
            recipes.push(recipe.clone());
        }
        else if next_i == recipe.len()-1 {
            // next ingredient is last, use all remaining space
            recipe[next_i] = remaining;
            Day15::gen_all_recurse(recipes, next_i+1, 0, recipe);
        }
        else {
            // Try all allocations of remaining space to this ingredient
            for amount in 0..=remaining {
                recipe[next_i] = amount;
                Day15::gen_all_recurse(recipes, next_i+1, remaining-amount, recipe);
            }
        }
    }

    fn gen_all_recipes(recipes: &mut Vec<Vec<i64>>, num_ingredients: usize, recipe_size: i64) -> &Vec<Vec<i64>> {
        let mut recipe = vec!{0; num_ingredients};
        Day15::gen_all_recurse(recipes, 0, recipe_size, &mut recipe);

        recipes
    }

    fn evaluate_recipe(&self, recipe: &Vec<i64>) -> [i64; 6] {
        let mut capacity: i64 = 0;
        let mut durability: i64 = 0;
        let mut flavor: i64 = 0;
        let mut texture: i64 = 0;
        let mut calories: i64 = 0;

        // evaluate components for this recipe
        for (i, amount) in recipe.iter().enumerate() {
            capacity += amount * self.ingredients[i].capacity;
            durability += amount * self.ingredients[i].durability;
            flavor += amount * self.ingredients[i].flavor;
            texture += amount * self.ingredients[i].texture;
            calories += amount * self.ingredients[i].calories;
        }

        // zero out any negative components
        let p_capacity = cmp::max(capacity, 0);
        let p_durability = cmp::max(durability, 0);
        let p_flavor = cmp::max(flavor, 0);
        let p_texture = cmp::max(texture, 0);

        [p_capacity * p_durability * p_flavor * p_texture, capacity, durability, flavor, texture, calories]
    }

    fn best_score(&self) -> i64 {
        let mut max_score = 0;
        for recipe in &self.all_recipes {
            let result = self.evaluate_recipe(recipe);
            max_score = cmp::max(max_score, result[0]);
        }

        max_score
    }

    fn best_500cal(&self) -> i64 {
        let mut max_score = 0;
        for recipe in &self.all_recipes {
            let result = self.evaluate_recipe(recipe);
            let calories = result[5];
            if calories == 500 {
                max_score = cmp::max(max_score, result[0]);
            }
        }

        max_score
    }
}

impl super::Day for Day15 {
    fn part1(&mut self) -> Result<i64, &str> {
        return Ok(self.best_score() as i64);
    }

    fn part2(&mut self) -> Result<i64, &str> {
        return Ok(self.best_500cal() as i64);
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

        // Butterscotch 44, Cinnamon 56 example.
        let result = d.evaluate_recipe(&vec!(44, 56));
        assert_eq!(62842880, result[0]);
    }

    #[test]
    fn test_best_cookie() {
        let d = Day15::load("data/day15_example1.txt");

        // Get best possible recipe and score it.
        assert_eq!(62842880, d.best_score());
    }

    #[test]
    fn test_part1() {
        let mut d = Day15::load("data/day15_input.txt");

        // Get best possible recipe and score it.
        let result = d.part1();
        assert_ne!(result, Ok(7453620));
        assert_eq!(result, Ok(21367368));
    }

    #[test]
    fn test_part2() {
        let mut d = Day15::load("data/day15_input.txt");

        // Get best possible recipe and score it.
        let result = d.part2();
        assert_eq!(result, Ok(1766400));
    }
}
