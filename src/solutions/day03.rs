use crate::solutions::Solution;
use crate::util::int;

pub struct Day {}

fn max_joltage(bank: &str, num_batteries: usize) -> i64 {
    let batteries: Vec<i64> = bank.chars().map(|c| int(&c.to_string()).unwrap()).collect();
    let len = batteries.len();
    let mut joltage = 0;
    let mut start_i = 0;
    for d in 1..=num_batteries {
        let best_digit = batteries[start_i..len - (num_batteries - d)]
            .iter()
            .max() // max_by_key returns rightmost maximum (annoying)
            .unwrap();
        let digit_i = batteries[start_i..]
            .iter()
            .enumerate()
            .find(|x| x.1 == best_digit)
            .unwrap()
            .0;
        joltage *= 10;
        joltage += best_digit;
        start_i += digit_i + 1; // digit_i is relative to the (potentially already shortened) start_i
    }
    joltage
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let ans: i64 = input.lines().map(|bank| max_joltage(bank, 2)).sum();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        let ans: i64 = input.lines().map(|bank| max_joltage(bank, 12)).sum();
        format!("{}", ans)
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
        assert_eq!(result2, "3121910778619");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(3);

        let result1 = d.part1(&input);
        assert_eq!(result1, "17452");

        let result2 = d.part2(&input);
        assert_eq!(result2, "173300819005913");
    }
}
