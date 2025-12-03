use fancy_regex::Regex;

use crate::solutions::Solution;
use crate::util::int;

pub struct Day {}

fn sum_invalids(range: Vec<&str>, re: &Regex) -> i64 {
    let low = int(range.get(0).unwrap()).unwrap();
    let high = int(range.get(1).unwrap()).unwrap();

    (low..=high)
        .filter(|x| re.is_match(&format!("{}", x)).unwrap_or(false))
        .sum()
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"^(\d+)\1$").unwrap();
        let ans: i64 = input
            .split(',')
            .map(|range| sum_invalids(range.split('-').collect(), &re))
            .sum();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"^(\d+)\1+$").unwrap();
        let ans: i64 = input
            .split(',')
            .map(|range| sum_invalids(range.split('-').collect(), &re))
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
