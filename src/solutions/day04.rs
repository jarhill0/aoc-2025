use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day {}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let grid = Grid::from(input);
        let ans = grid
            .rolls
            .iter()
            .filter(|(x, y)| {
                Grid::adjacent(*x, *y)
                    .iter()
                    .filter(|(x1, y1)| grid.rolls.contains(&(*x1, *y1)))
                    .count()
                    < 4
            })
            .count();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
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
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(4);

        let result1 = d.part1(&input);
        assert_eq!(result1, "1569");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
