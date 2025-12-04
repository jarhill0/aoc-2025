use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day {}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let grid = Grid::from(input);
        let ans = grid.to_remove().len();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = Grid::from(input);
        let ans = grid.remove_all();
        format!("{}", ans)
    }
}

struct Grid {
    rolls: HashSet<(usize, usize)>,
}

impl Grid {
    fn from(inp: &str) -> Grid {
        let mut rolls = HashSet::new();

        inp.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '@' {
                    rolls.insert((x, y));
                }
            })
        });

        Grid { rolls }
    }

    fn adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut out = vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)];
        if x > 0 {
            out.push((x - 1, y));
            out.push((x - 1, y + 1))
        }
        if y > 0 {
            out.push((x, y - 1));
            out.push((x + 1, y - 1));
        }
        if x > 0 && y > 0 {
            out.push((x - 1, y - 1));
        }
        out
    }

    fn remove_all(&mut self) -> usize {
        let mut total = 0;
        loop {
            let removed = self.remove();
            if removed == 0 {
                break;
            }
            total += removed;
        }
        total
    }

    fn to_remove(&self) -> Vec<(usize, usize)> {
        self.rolls
            .iter()
            .filter(|(x, y)| {
                Grid::adjacent(*x, *y)
                    .iter()
                    .filter(|(x1, y1)| self.rolls.contains(&(*x1, *y1)))
                    .count()
                    < 4
            })
            .map(|(x, y)| (*x, *y))
            .collect()
    }

    fn remove(&mut self) -> usize {
        let to_remove = self.to_remove();
        to_remove.iter().for_each(|roll| {
            self.rolls.remove(roll);
        });

        to_remove.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "13");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "43");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(4);

        let result1 = d.part1(&input);
        assert_eq!(result1, "1569");

        let result2 = d.part2(&input);
        assert_eq!(result2, "9280");
    }
}
