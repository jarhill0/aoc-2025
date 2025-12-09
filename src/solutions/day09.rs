use crate::solutions::Solution;
use crate::util::int;
use std::collections::HashSet;

pub struct Day {}

fn parse(inp: &str) -> Vec<(i64, i64)> {
    inp.lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (int(x).unwrap(), int(y).unwrap())
        })
        .collect()
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let pts = parse(input);
        let ans = pts
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                pts[i + 1..]
                    .iter()
                    .map(|p2| (i64::abs(p1.0 - p2.0) + 1) * (i64::abs(p1.1 - p2.1) + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0);
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let pts = parse(input);
        let in_bounds = fill(&pts);
        dbg!(in_bounds.len());

        let ans = pts
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                pts[i + 1..]
                    .iter()
                    .filter(|p2| {
                        let (xmin, xmax) = twosort(p1.0, p2.0);
                        let (ymin, ymax) = twosort(p1.1, p2.1);
                        (xmin..=xmax).all(|x| (ymin..=ymax).all(|y| in_bounds.contains(&(x, y))))
                    })
                    .map(|p2| (i64::abs(p1.0 - p2.0) + 1) * (i64::abs(p1.1 - p2.1) + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0);

        format!("{}", ans)
    }
}

fn fill(pts: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut out = HashSet::new();
    out.extend(line_between(pts.last().unwrap(), pts.first().unwrap()));
    pts.iter().zip(pts[1..].iter()).for_each(|(p1, p2)| {
        out.extend(line_between(p1, p2));
    });

    let left_boundary = *out.iter().map(|(x, _)| x).min().unwrap();
    let left_corners: Vec<&(i64, i64)> = pts
        .iter()
        .filter(|(x, _)| *x == left_boundary)
        .take(2)
        .collect();
    let (lower, _upper) = twosort(left_corners[0].1, left_corners[1].1);

    let start_pt = (left_boundary + 1, lower + 1);
    let mut stack = vec![start_pt];
    out.insert(start_pt);
    while !stack.is_empty() {
        let pt = stack.pop().unwrap();
        neighbors(pt).iter().for_each(|n| {
            if !out.contains(n) {
                out.insert(*n);
                stack.push(*n);
            }
        });
        println!("{} {}", out.len(), stack.len());
    }

    out
}

fn neighbors(pt: (i64, i64)) -> [(i64, i64); 4] {
    let (x, y) = pt;
    [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
}

fn twosort(a: i64, b: i64) -> (i64, i64) {
    if a < b { (a, b) } else { (b, a) }
}

fn line_between(a: &(i64, i64), b: &(i64, i64)) -> Vec<(i64, i64)> {
    match (a.0 == b.0, a.1 == b.1) {
        (true, true) => vec![*a],
        (true, false) => {
            let (small, big) = twosort(a.1, b.1);
            (small..=big).map(|y| (a.0, y)).collect()
        }
        (false, true) => {
            let (small, big) = twosort(a.0, b.0);
            (small..=big).map(|x| (x, a.1)).collect()
        }
        (false, false) => {
            panic!("bad input")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "50");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "24");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(9);

        let result1 = d.part1(&input);
        assert_eq!(result1, "4774877510");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
