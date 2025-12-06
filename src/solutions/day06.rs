use crate::solutions::Solution;
use crate::util::{int, token_lines};

pub struct Day {}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let problems = token_lines(input);
        let mut total = 0;
        for p in (0..problems[0].len()) {
            let mut nums = Vec::new();
            for i in (0..problems.len() - 1) {
                nums.push(int(problems[i][p]).unwrap())
            }
            match problems[problems.len() - 1][p] {
                "*" => total += nums.iter().product::<i64>(),
                "+" => total += nums.iter().sum::<i64>(),
                _ => panic!("unknown operation"),
            }
        }
        format!("{}", total)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "4277556");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(6);

        let result1 = d.part1(&input);
        assert_eq!(result1, "4722948564882");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
