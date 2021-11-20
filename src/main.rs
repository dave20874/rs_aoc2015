mod nql;
mod presents;
mod houses;
mod mining;
mod naughty_nice;
mod lights;
mod logic;
mod matchsticks;
mod traveler;
mod look_say;
mod password;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

use nql::NotQuiteLisp;
use presents::Presents;
use houses::Houses;
use mining::Mining;
use naughty_nice::NaughtyNice;
use lights::Lights;
use logic::Circuit;
use matchsticks::Matchsticks;
use traveler::Traveler;
use look_say::LookSay;
use password::PwGen;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&mut self) -> Result<i64, &str> ;
    fn part2(&mut self) -> Result<i64, &str> ;
}

pub fn run(n: Option<usize>) {
    // Create array of days.  Each entry references a Day.
    let mut day1 = NotQuiteLisp::load("data/day1_input.txt");
    let mut day2 = Presents::load("data/day2_input.txt");
    let mut day3 = Houses::load("data/day3_input.txt");
    let mut day4 = Mining::new("iwrupvqb");
    let mut day5 = NaughtyNice::load("data/day5_input.txt");
    let mut day6 = Lights::load("data/day6_input.txt");
    let mut day7 = Circuit::load("data/day7_input.txt");
    let mut day8 = Matchsticks::load("data/day8_input.txt");
    let mut day9 = Traveler::load("data/day9_input.txt");
    let mut day10 = LookSay::new("113122113");
    let mut day11 = PwGen::new("cqjxjnds");
    let mut day12 = Day12::load("data/day12_input.txt");
    let mut day13 = Day13::load("data/day13_input.txt");
    let mut day14 = Day14::load("data/day14_input.txt");
    let mut day15 = Day15::load("data/day15_input.txt");
    let mut day16 = Day16::load("data/day16_input.txt");
    let mut day17 = Day17::load("data/day17_input.txt");
    let mut day18 = Day18::load("data/day18_input.txt");
    let mut day19 = Day19::load("data/day19_input.txt");

    let mut days: Vec<&mut dyn Day> = vec![
        &mut day1,
        &mut day2,
        &mut day3,
        &mut day4,
        &mut day5,
        &mut day6,
        &mut day7,
        &mut day8,
        &mut day9,
        &mut day10,
        &mut day11,
        &mut day12,
        &mut day13,
        &mut day14,
        &mut day15,
        &mut day16,
        &mut day17,
        &mut day18,
        &mut day19,
    ];

    match n {
        Some(day_no) => {
            // Run for one day.
            match days[day_no-1].part1() {
                Ok(val) => println!("Day {}, part 1: {}", day_no, val),
                Err(_) => println!("Day {}, part 1: No result.", day_no),
            }

            match days[day_no-1].part2() {
                Ok(val) => println!("Day {}, part 2: {}", day_no, val),
                Err(_) => println!("Day {}, part 2: No result.", day_no),
            }
        }
        None => {
            // Run for all days.
            for (n, day) in days.iter_mut().enumerate()
            {
                let day_no = n+1;

                match day.part1() {
                    Ok(val) => println!("Day {}, part 1: {}", day_no, val),
                    Err(_) => println!("Day {}, part 1: No result.", day_no),
                }

                match day.part2() {
                    Ok(val) => println!("Day {}, part 2: {}", day_no, val),
                    Err(_) => println!("Day {}, part 2: No result.", day_no),
                }
            }
        }
    }
}

fn main() {
    println!("Advent of Code 2015.");

    run(Some(19));
}


