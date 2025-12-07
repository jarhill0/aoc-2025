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
        let (start, splits, max_r) = parse(input);
        let mut row = HashMap::new();
        row.insert(start, 1i64);
        while row.keys().next().unwrap().r < max_r {
            // visualize(&row, &splits);

            let mut next_row = HashMap::new();
            row.iter().for_each(|(p, ways_to_get)| {
                let b = p.below();
                if splits.contains(&b) {
                    inc_key_by(&mut next_row, b.left(), *ways_to_get);
                    inc_key_by(&mut next_row, b.right(), *ways_to_get);
                } else {
                    inc_key_by(&mut next_row, b, *ways_to_get);
                }
            });

            row = next_row;
        }

        format!("{}", row.values().sum::<i64>())
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

fn visualize(row_ways: &HashMap<Point, i64>, splits: &HashSet<Point>) {
    let row = row_ways.keys().next().unwrap().r;
    let row_ways: HashMap<i64, i64> = row_ways.iter().map(|(p, v)| (p.c, *v)).collect();
    let splits: HashSet<i64> = splits.iter().filter(|p| p.r == row).map(|p| p.c).collect();
    let max_c = *row_ways.keys().chain(splits.iter()).max().unwrap_or(&0);

    let mut out = Vec::new();
    (0..=max_c).for_each(|c| {
        if splits.contains(&c) {
            out.push('^');
        } else if let Some(ways) = row_ways.get(&c) {
            out.push(stupid(*ways));
        } else {
            out.push(' ');
        }
    });

    println!("{}", out.iter().collect::<String>());
}

fn stupid(x: i64) -> char {
    if x < 10 {
        (('0' as u8) + x as u8) as char
    } else if x < 10 + 26 {
        (('a' as u8) + (x - 10) as u8) as char
    } else if x < 10 + 26 + 26 {
        (('A' as u8) + (x - 10 - 26) as u8) as char
    } else {
        panic!("I didn't think")
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
