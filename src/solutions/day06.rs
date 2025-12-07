use crate::solutions::Solution;
use crate::util::{int, token_lines, tokens};

pub struct Day {}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let problems = token_lines(input);
        let mut total = 0;
        for p in 0..problems[0].len() {
            let mut nums = Vec::new();
            for row in problems[..problems.len() - 1].iter() {
                nums.push(int(row[p]).unwrap())
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
        let mut lines: Vec<&str> = input.lines().collect();
        let operators = tokens(lines[lines.len() - 1]);
        lines.remove(lines.len() - 1);
        let lines = lines;
        let reflected = reflect(&lines);
        let reflected: Vec<Result<i64, _>> = reflected.iter().map(|n| int(n)).collect();

        let number_groups = reflected.split(|n| n.is_err());
        let total: i64 = number_groups
            .map(|g| g.iter().map(|n| n.clone().unwrap()).collect::<Vec<i64>>())
            .zip(operators.iter())
            .map(|(nums, op)| prob(&nums, op))
            .sum();

        format!("{}", total)
    }
}

fn prob(nums: &[i64], op: &str) -> i64 {
    match op {
        "*" => nums.iter().product(),
        "+" => nums.iter().sum(),
        _ => panic!("unknown operation"),
    }
}

fn reflect(orig: &[&str]) -> Vec<String> {
    let width = orig.iter().map(|x| x.len()).max().unwrap();

    (0..width)
        .map(|origc| {
            orig.iter()
                .map(|orig_row| orig_row.chars().nth(origc).unwrap_or(' '))
                .collect()
        })
        .collect()
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
        assert_eq!(result2, "3263827");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(6);

        let result1 = d.part1(&input);
        assert_eq!(result1, "4722948564882");

        let result2 = d.part2(&input);
        assert_eq!(result2, "9581313737063");
    }
}
