use crate::solutions::Solution;
use crate::util::int;
use std::fs::write;

pub struct Day {}

fn max_joltage(bank: &str) -> i64 {
    let batteries: Vec<i64> = bank.chars().map(|c| int(&c.to_string()).unwrap()).collect();
    let len = batteries.len();
    let first_digit = batteries[..len - 1]
        .iter()
        .max() // max_by_key returns rightmost maximum (annoying)
        .unwrap();
    let first_i = batteries
        .iter()
        .enumerate()
        .filter(|x| x.1 == first_digit)
        .next()
        .unwrap()
        .0;
    let second_digit = batteries[first_i + 1..].iter().max().unwrap();
    first_digit * 10 + second_digit
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let ans: i64 = input.lines().map(max_joltage).sum();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "357");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(3);

        let result1 = d.part1(&input);
        assert_ne!(result1, "17298");
        assert_eq!(result1, "");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
