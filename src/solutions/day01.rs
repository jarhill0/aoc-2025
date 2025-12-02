use crate::solutions::Solution;
use crate::util::int;

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        parse(input).iter().fold([50, 0], |acc, mv| {
            let [mut pos, mut zero_count] = acc;
            match mv {
                Move::Left(amt) => pos = (pos - amt) % 100,
                Move::Right(amt) => pos = (pos + amt) % 100,
            }
            if pos == 0 {
                zero_count += 1;
            }
            [pos, zero_count]
        })[1]
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse(input).iter().fold([50, 0], |acc, mv| {
            let [mut pos, mut zero_count] = acc;
            let move_amt = match mv {
                Move::Left(amt) => {
                    zero_count += amt / 100;
                    (-amt) % 100 // actually using the annoying Rust mod behavior
                }
                Move::Right(amt) => {
                    zero_count += amt / 100;
                    amt % 100
                }
            };
            if pos + move_amt >= 100 || (pos > 0 && pos + move_amt <= 0) {
                zero_count += 1
            }
            pos += move_amt;
            pos = ((pos % 100) + 100) % 100;
            [pos, zero_count]
        })[1]
            .to_string()
    }
}

#[derive(Debug)]
enum Move {
    Left(i64),
    Right(i64),
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|m| match m.chars().next().unwrap_or('X') {
            'L' => Move::Left(int(&m[1..]).expect("input should have a number")),
            'R' => Move::Right(int(&m[1..]).expect("input should have a number")),
            _ => panic!("unexpected move type"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn example() {
        let d = Day1 {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "3");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "6");
    }

    #[test]
    fn actual() {
        let d = Day1 {};
        let input = crate::input(1);

        let result1 = d.part1(&input);
        assert_eq!(result1, "1034");

        let result2 = d.part2(&input);
        assert_eq!(result2, "6166");
    }
}
