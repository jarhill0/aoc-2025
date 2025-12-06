use crate::solutions::Solution;
use crate::util::{int, token_lines, tokens};

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
        let mut lines: Vec<&str> = input.lines().collect();
        let operators = tokens(lines[lines.len() - 1]);
        lines.remove(lines.len() - 1);
        let re_parsed = re_parse(&lines);

        let mut op_ind = 0;
        let mut total = 0;
        let mut nums = Vec::new();
        re_parsed.iter().for_each(|n| {
            dbg!(n);
            if let Ok(n) = int(n.trim()) {
                nums.push(n);
            } else {
                total += prob(&nums, operators[op_ind]);
                nums.clear();
                op_ind += 1;
                dbg!("adding it up!", op_ind);
            }
        });
        if nums.len() > 0 {
            total += prob(&nums, operators[op_ind]);
        }

        format!("{}", total)
    }
}

fn prob(nums: &Vec<i64>, op: &str) -> i64 {
    match op {
        "*" => nums.iter().product(),
        "+" => nums.iter().sum(),
        _ => panic!("unknown operation"),
    }
}

fn re_parse(orig: &Vec<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let width = orig.iter().map(|x| x.len()).max().unwrap();
    for origc in (0..width) {
        let mut out_row = Vec::new();
        for origr in (0..orig.len()) {
            out_row.push(orig[origr].chars().nth(origc).unwrap_or(' '))
        }
        out.push(out_row.iter().collect())
    }
    out
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
        assert_eq!(result2, "");
    }
}
