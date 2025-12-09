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
        let p1ans = int(&self.part1(input)).unwrap(); // 🤡

        let pts = parse(input);
        let perimeter = boundary(&pts);
        dbg!(perimeter.len());
        let (bound_min, bound_max) = bounding_box(&pts);
        let point_inside = |pt: &(i64, i64)| {
            if perimeter.contains(pt) {
                return true;
            }
            let mut inside = false;
            (bound_min.1 - 1..pt.1).for_each(|y| {
                if perimeter.contains(&(pt.0, y)) {
                    inside = !inside;
                }
            });
            inside
        };

        let ans = pts
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                dbg!(i);
                pts[i + 1..]
                    .iter()
                    .filter(|p2| (i64::abs(p1.0 - p2.0) + 1) * (i64::abs(p1.1 - p2.1) + 1) < p1ans)
                    .filter(|p2| {
                        dbg!(p2);
                        let (xmin, xmax) = twosort(p1.0, p2.0);
                        let (ymin, ymax) = twosort(p1.1, p2.1);

                        // let inside_contains_perimeter = (xmin + 1..xmax)
                        //     .any(|x| (ymin + 1..ymax).any(|y| perimeter.contains(&(x, y))));
                        let inside_contains_perimeter = perimeter
                            .iter()
                            .any(|(x, y)| xmin < *x && *x < xmax && ymin < *y && *y < ymax);

                        !inside_contains_perimeter
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

fn boundary(pts: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut out = HashSet::new();
    out.extend(line_between(pts.last().unwrap(), pts.first().unwrap()));
    pts.iter().zip(pts[1..].iter()).for_each(|(p1, p2)| {
        out.extend(line_between(p1, p2));
    });

    out
}

fn bounding_box(pts: &[(i64, i64)]) -> ((i64, i64), (i64, i64)) {
    let xmin = pts.iter().map(|p| p.0).min().unwrap();
    let xmax = pts.iter().map(|p| p.0).max().unwrap();
    let ymin = pts.iter().map(|p| p.1).min().unwrap();
    let ymax = pts.iter().map(|p| p.1).max().unwrap();
    ((xmin, ymin), (xmax, ymax))
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
        assert_eq!(result2, "1560475800");
    }
}
