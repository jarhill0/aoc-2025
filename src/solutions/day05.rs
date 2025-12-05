use crate::solutions::Solution;
use crate::util::{int, ints};
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct Day {}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let (ranges, nums) = parse(input);
        let ans = nums
            .iter()
            .filter(|n| ranges.iter().any(|r| r.contains(n)))
            .count();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        // I swear a problem in a previous year also involved building an efficient "range-set" data structure
        let (ranges, _) = parse(input);
        let mut ranges: HashSet<RangeInclusive<i64>> = HashSet::from_iter(ranges.iter().cloned());
        loop {
            let mut remove = Vec::new();
            let mut add = Vec::new();
            ranges.iter().for_each(|r1| {
                ranges.iter().filter(|r2| r1 != *r2).for_each(|r2| {
                    if r2.contains(r1.start()) && remove.len() == 0 {
                        remove.push(r1.clone());
                        if !r2.contains(r1.end()) {
                            add.push((r2.end() + 1)..=*r1.end())
                        }
                    }
                })
            });

            if remove.len() == 0 && add.len() == 0 {
                break;
            }

            remove.iter().for_each(|r| {
                ranges.remove(r);
            });
            add.iter().for_each(|r| {
                ranges.insert(r.clone());
            })
        }

        let ans: i64 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
        format!("{}", ans)
    }
}

fn parse(inp: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let (ranges, nums) = inp.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (l, u) = line.split_once('-').unwrap();
            let l = int(l).unwrap();
            let u = int(u).unwrap();
            l..=u
        })
        .collect();
    let nums = ints(nums);
    (ranges, nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "3");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "14");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(5);

        let result1 = d.part1(&input);
        assert_eq!(result1, "511");

        let result2 = d.part2(&input);
        assert_eq!(result2, "350939902751909");
    }
}
