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
        let (ranges, _) = parse(input);
        let mut fresh = HashSet::new();
        ranges.iter().for_each(|r| {
            r.clone().for_each(|n| {
                fresh.insert(n);
            })
        });
        format!("{}", fresh.len())
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
        assert_eq!(result2, "");
    }
}
