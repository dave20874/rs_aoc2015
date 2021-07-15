mod nql;
mod presents;

use nql::NotQuiteLisp;
use presents::Presents;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&self) -> Result<i64, &str> ;
    fn part2(&self) -> Result<i64, &str> ;
}

pub fn run(n: Option<usize>) {
    // Create array of days.  Each entry references a Day.
    let day1 = NotQuiteLisp::load("data/day1_input.txt");
    let day2 = Presents::load("data/day2_input.txt");
    let days: Vec<&dyn Day> = vec![
        &day1,
        &day2,
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

    run(None);
}


