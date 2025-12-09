use crate::solutions::Solution;
use crate::util::int;

pub struct Day {}

fn parse(inp: &str) -> Vec<(i64, i64)> {
    inp.lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (int(x).unwrap(), int(y).unwrap())
        })
        .collect()
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let pts = parse(input);
        let ans = pts
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                pts[i + 1..]
                    .iter()
                    .map(|p2| (i64::abs(p1.0 - p2.0) + 1) * (i64::abs(p1.1 - p2.1) + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0);
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "50");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(9);

        let result1 = d.part1(&input);
        assert_eq!(result1, "4774877510");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
