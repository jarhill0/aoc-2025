use std::collections::HashSet;

use crate::solutions::Solution;
use crate::util::int;

pub struct Day {}

fn sum_invalids(range: Vec<&str>) -> i64 {
    let low = int(range.first().unwrap()).unwrap();
    let high = int(range.get(1).unwrap()).unwrap();
    let d = digits(high) / 2;

    let mut rpt_portion = low;
    for _ in 0..d {
        rpt_portion /= 10;
    }
    let rpt_portion = rpt_portion;

    let mut total = 0;
    for repeat_digits in rpt_portion.. {
        if digits(repeat_digits) > d {
            break;
        }
        if digits(repeat_digits) < d {
            continue;
        }
        let mut candidate = repeat_digits;
        for _ in 0..d {
            candidate *= 10
        }
        candidate += repeat_digits;
        let candidate = candidate;

        if candidate > high {
            break;
        }
        if low <= candidate {
            total += candidate
        }
    }
    total
}

fn digits(mut x: i64) -> i64 {
    if x < 0 {
        x = -x
    }
    let mut count = 0;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
}

fn sum_invalids2(invalids: &HashSet<i64>, range: Vec<&str>) -> i64 {
    let low = int(range.first().unwrap()).unwrap();
    let high = int(range.get(1).unwrap()).unwrap();

    (low..=high).filter(|x| invalids.contains(x)).sum()
}

fn digit_left_shift(mut x: i64, by: i64) -> i64 {
    for _ in 0..by {
        x *= 10;
    }
    x
}

fn all_invalids2(n: i64) -> HashSet<i64> {
    let mut set = HashSet::new();

    for d in 1..=n {
        for rpt_len in 1..=(d / 2) {
            if d % rpt_len != 0 {
                continue;
            }

            let segment_start = digit_left_shift(1, rpt_len - 1); // e.g. 100
            let segment_end = digit_left_shift(1, rpt_len) - 1; // e.g. 999
            for segment in segment_start..=segment_end {
                let mut candidate = 0;
                for _ in 0..(d / rpt_len) {
                    candidate = digit_left_shift(candidate, rpt_len) + segment
                }
                set.insert(candidate);
            }
        }
    }

    set
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let ans: i64 = input
            .split(',')
            .map(|range| sum_invalids(range.split('-').collect()))
            .sum();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let invalids = all_invalids2(12); // this is really silly lmao
        let ans: i64 = input
            .split(',')
            .map(|range| sum_invalids2(&invalids, range.split('-').collect()))
            .sum();
        format!("{}", ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "1227775554");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "4174379265");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(2);

        let result1 = d.part1(&input);
        assert_eq!(result1, "18595663903");

        let result2 = d.part2(&input);
        assert_eq!(result2, "19058204438");
    }
}
