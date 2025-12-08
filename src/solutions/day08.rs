use crate::solutions::Solution;
use crate::util::int;
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct Day {}

type Point = (i64, i64, i64);

fn parse(inp: &str) -> Vec<Point> {
    inp.lines()
        .map(|l| {
            let l: Vec<i64> = l.split(',').map(|x| int(x).unwrap()).collect();
            (l[0], l[1], l[2])
        })
        .collect()
}

fn pairs_by_distance(points: &[Point]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = (0..points.len())
        .flat_map(|p1id| (p1id + 1..points.len()).map(move |p2id| (p1id, p2id)))
        .collect();

    pairs.sort_by_key(|(p1id, p2id)| dist(points[*p1id], points[*p2id]));
    pairs.reverse();

    pairs
}

#[derive(PartialOrd, PartialEq)]
struct MyF64Lmao {
    v: f64,
}
impl Eq for MyF64Lmao {}
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for MyF64Lmao {
    // very silly. But I trust my distance function to always return comparable values
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("you have been cursed")
    }
}

fn dist(a: Point, b: Point) -> MyF64Lmao {
    let xd = (a.0 - b.0) as f64;
    let yd = (a.1 - b.1) as f64;
    let zd = (a.2 - b.2) as f64;
    MyF64Lmao {
        v: f64::sqrt(xd * xd + yd * yd + zd * zd),
    }
}

fn connected(connections: &[HashSet<usize>], a: usize, b: usize) -> bool {
    connections.iter().any(|s| s.contains(&a) && s.contains(&b))
}

fn connect(connections: &mut Vec<HashSet<usize>>, pair: (usize, usize)) {
    let (a, b) = pair;
    if connected(connections, a, b) {
        return;
    }

    let a_set_ind = connections
        .iter()
        .enumerate()
        .find(|(_, s)| s.contains(&a))
        .map(|(i, _)| i);
    let b_set_ind = connections
        .iter()
        .enumerate()
        .find(|(_, s)| s.contains(&b))
        .map(|(i, _)| i);

    match (a_set_ind, b_set_ind) {
        (None, None) => {
            connections.push(HashSet::from([a, b]));
        }
        (Some(ind), None) | (None, Some(ind)) => {
            let set = &mut connections[ind];
            set.insert(a);
            set.insert(b);
        }
        (Some(a_ind), Some(b_ind)) => {
            let (larger_set_ind, smaller_set_ind) =
                if connections[a_ind].len() > connections[b_ind].len() {
                    (a_ind, b_ind)
                } else {
                    (b_ind, a_ind)
                };
            let smaller_set = connections.remove(smaller_set_ind);
            let larger_set = &mut connections[larger_set_ind];
            larger_set.extend(smaller_set);
        }
    }
}

impl Day {
    fn solve(&self, input: &str, part1_iterations: i64) -> (usize, i64) {
        let points = parse(input);
        let mut connected_subsets: Vec<HashSet<usize>> =
            (0..points.len()).map(|id| HashSet::from([id])).collect();
        let mut ordered_pairs = pairs_by_distance(&points);
        let mut final_pair_product = 0;
        let mut part1_ans = 0;
        let mut iterations = 0;

        while connected_subsets.len() != 1 {
            let pair = ordered_pairs.pop().unwrap();
            connect(&mut connected_subsets, pair);
            final_pair_product = points[pair.0].0 * points[pair.1].0;

            iterations += 1;
            if iterations == part1_iterations {
                let mut subset_sizes: Vec<usize> =
                    connected_subsets.iter().map(|s| s.len()).collect();
                subset_sizes.sort();
                subset_sizes.reverse();
                part1_ans = subset_sizes.iter().take(3).product();
            }
        }

        (part1_ans, final_pair_product)
    }
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let num_connections = if input.lines().count() <= 20 {
            10 // example
        } else {
            1000 // actual
        };
        let (ans, _) = self.solve(input, num_connections);
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let (_, ans) = self.solve(input, 0);
        format!("{}", ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "40");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "25272");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(8);

        let result1 = d.part1(&input);
        assert_eq!(result1, "57564");

        let result2 = d.part2(&input);
        assert_eq!(result2, "133296744");
    }
}
