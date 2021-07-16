mod nql;
mod presents;
mod houses;
mod mining;
mod naughty_nice;
mod lights;

use nql::NotQuiteLisp;
use presents::Presents;
use houses::Houses;
use mining::Mining;
use naughty_nice::NaughtyNice;
use lights::Lights;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&self) -> Result<i64, &str> ;
    fn part2(&self) -> Result<i64, &str> ;
}

pub fn run(n: Option<usize>) {
    // Create array of days.  Each entry references a Day.
    let day1 = NotQuiteLisp::load("data/day1_input.txt");
    let day2 = Presents::load("data/day2_input.txt");
    let day3 = Houses::load("data/day3_input.txt");
    let day4 = Mining::new("iwrupvqb");
    let day5 = NaughtyNice::load("data/day5_input.txt");
    let day6 = Lights::load("data/day6_input.txt");
    let days: Vec<&dyn Day> = vec![
        &day1,
        &day2,
        &day3,
        &day4,
        &day5,
        &day6,
    ];

    match n {
        Some(day_no) => {
            // Run for one day.
            match days[day_no-1].part1() {
                Ok(val) => println!("Day {}, part 1: {}", day_no, val),
                Err(_) => println!("Day {}, part 1: No result found.", day_no),
            }

            match days[day_no-1].part2() {
                Ok(val) => println!("Day {}, part 2: {}", day_no, val),
                Err(_) => println!("Day {}, part 2: No result found.", day_no),
            }
        }
        None => {
            // Run for all days.
            for (n, day) in days.iter().enumerate()
            {
                let day_no = n+1;

                match day.part1() {
                    Ok(val) => println!("Day {}, part 1: {}", day_no, val),
                    Err(_) => println!("Day {}, part 1: No result found.", day_no),
                }

                match day.part2() {
                    Ok(val) => println!("Day {}, part 2: {}", day_no, val),
                    Err(_) => println!("Day {}, part 2: No result found.", day_no),
                }
            }
        }
    }
}

fn main() {
    println!("Advent of Code 2015.");

    run(Some(6));
}


