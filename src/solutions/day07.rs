use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day {}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    r: i64,
    c: i64,
}

impl Point {
    fn below(&self) -> Point {
        Point {
            r: self.r + 1,
            c: self.c,
        }
    }
    fn left(&self) -> Point {
        Point {
            r: self.r,
            c: self.c - 1,
        }
    }
    fn right(&self) -> Point {
        Point {
            r: self.r,
            c: self.c + 1,
        }
    }
}
fn parse(inp: &str) -> (Point, HashSet<Point>, i64) {
    let mut start = None;
    let mut splits = HashSet::new();
    let max_r = inp.lines().count() as i64;

    inp.lines().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            let p = Point {
                r: r as i64,
                c: c as i64,
            };
            match ch {
                'S' => start = Some(p),
                '^' => {
                    splits.insert(p);
                }
                _ => (),
            }
        })
    });

    (start.unwrap(), splits, max_r)
}
impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let (start, splits, max_r) = parse(input);
        let mut total_splits = 0;
        let mut row = HashSet::new();
        row.insert(start);
        while row.iter().next().unwrap().r < max_r {
            let mut next_row = HashSet::new();
            row.iter().for_each(|p| {
                let b = p.below();
                if splits.contains(&b) {
                    next_row.insert(b.left());
                    next_row.insert(b.right());
                    total_splits += 1;
                } else {
                    next_row.insert(b);
                }
            });

            row = next_row;
        }

        format!("{}", total_splits)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "21");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(7);

        let result1 = d.part1(&input);
        assert_eq!(result1, "1687");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
