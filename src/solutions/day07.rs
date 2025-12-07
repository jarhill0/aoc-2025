use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

pub struct Day {}

#[derive(Eq, PartialEq, Hash, Debug)]
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

impl Day {
    fn solve(&self, input: &str) -> (i64, i64) {
        let (start, splits, max_r) = parse(input);
        let mut total_splits = 0;
        let mut row = HashMap::new();
        row.insert(start, 1i64);
        while row.keys().next().unwrap().r < max_r {
            let mut next_row = HashMap::new();
            row.iter().for_each(|(p, ways_to_get)| {
                let b = p.below();
                if splits.contains(&b) {
                    total_splits += 1;
                    inc_key_by(&mut next_row, b.left(), *ways_to_get);
                    inc_key_by(&mut next_row, b.right(), *ways_to_get);
                } else {
                    inc_key_by(&mut next_row, b, *ways_to_get);
                }
            });

            row = next_row;
        }

        (total_splits, row.values().sum::<i64>())
    }
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let (total_splits, _) = self.solve(input);

        format!("{}", total_splits)
    }

    fn part2(&self, input: &str) -> String {
        let (_, total_paths) = self.solve(input);

        format!("{}", total_paths)
    }
}

fn inc_key_by<K, V>(d: &mut HashMap<K, V>, k: K, v: V)
where
    K: Hash,
    K: Eq,
    V: Add<Output = V>,
    V: Clone,
{
    // lmao why the hell did I make this generic?
    let existing_val = d.get(&k);
    if let Some(existing_val) = existing_val {
        d.insert(k, existing_val.clone() + v);
    } else {
        d.insert(k, v);
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
        assert_eq!(result2, "40");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(7);

        let result1 = d.part1(&input);
        assert_eq!(result1, "1687");

        let result2 = d.part2(&input);
        assert_eq!(result2, "390684413472684");
    }
}
