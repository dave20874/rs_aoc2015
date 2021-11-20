use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day18 {
    grid: [[bool; 100]; 100],
}

impl Day18 {
    pub fn load(filename: &str) -> Day18 {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut grid = [[false; 100]; 100];

        let mut y = 0;
        while y < 100 {
            // process line
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            // println!("Read {}", line);
            let mut char_iter = line.chars();
            let mut x = 0;
            while x < 100 {
                if char_iter.next().unwrap() == '#' {
                    grid[x][y] = true;
                }
                x += 1;
            }
            y += 1;
        }

        Day18 {grid: grid}
    }

    fn neighbors_on(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for (dx, dy) in [(-1, -1), (0, -1), (1, -1),
                         (-1, 0),           (1, 0),
                         (-1, 1), (0, 1), (1, 1)] {
            let x_new = x as i32 + dx;
            let y_new = y as i32 + dy;
            if (x_new >= 0) && (x_new < 100) & (y_new >= 0) & (y_new < 100) {
                if self.grid[x_new as usize][y_new as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    fn update(&mut self, generations: usize, corners_on: bool) {
        let mut new_grid = [[false; 100]; 100];

        // in part 2, the corner cells are always on
        if corners_on {
            self.grid[0][0] = true;
            self.grid[99][0] = true;
            self.grid[0][99] = true;
            self.grid[99][99] = true;
        }

        for _ in 0..generations {
            for y in 0..100 {
                for x in 0..100 {
                    let neighbors_on = self.neighbors_on(x, y);
                    if self.grid[x][y] {
                        // was on.  stay on if 2 or 3 neighbors are on.
                        if (neighbors_on == 2) || (neighbors_on == 3) {
                            // stay on
                            new_grid[x][y] = true;
                        }
                        else {
                            // turns off
                            new_grid[x][y] = false;
                        }
                    }
                    else {
                        // was off.  turn on if exactly 3 neighbors.
                        if neighbors_on == 3 {
                            // turn on on
                            new_grid[x][y] = true;
                        }
                        else {
                            // turns off
                            new_grid[x][y] = false;
                        }
                    }
                }
            }

            // copy new generation into original grid
            for y in 0..100 {
                for x in 0..100 {
                    self.grid[x][y] = new_grid[x][y];
                }
            }

            // in part 2, the corner cells are always on
            if corners_on {
                self.grid[0][0] = true;
                self.grid[99][0] = true;
                self.grid[0][99] = true;
                self.grid[99][99] = true;
            }
        }
    }

    fn num_set(&self) -> usize {
        let mut count = 0;
        for y in 0..100 {
            for x in 0..100 {
                if self.grid[x][y] {
                    count += 1;
                }
            }
        }

        count
    }
}

impl super::Day for Day18 {
    fn part1(&mut self) -> Result<i64, &str> {
        self.update(100, false);
        Ok(self.num_set() as i64)
    }

    fn part2(&mut self) -> Result<i64, &str> {
        self.update(100, true);
        Ok(self.num_set() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let d = Day18::load("data/day18_input.txt");
        assert_eq!(d.num_set(), 4905);
    }

    #[test]
    fn test_update() {
        let mut d = Day18::load("data/day18_input.txt");
        d.update(0, false);
        assert_eq!(d.num_set(), 4905);
        d.update(1, false);
        assert_eq!(d.num_set(), 2922);

        let mut d = Day18::load("data/day18_input.txt");
        d.update(0, true);
        assert_eq!(d.num_set(), 4906);
        d.update(1, false);
        assert_eq!(d.num_set(), 2920);
    }

    #[test]
    fn test_part1() {
        let mut d = Day18::load("data/day18_input.txt");
        assert_eq!(d.part1(), Ok(821));
    }

    #[test]
    fn test_part2() {
        let mut d = Day18::load("data/day18_input.txt");
        assert_eq!(d.part2(), Ok(886));
    }
}

