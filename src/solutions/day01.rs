use crate::solutions::Solution;
use crate::util::int;

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self, input: String) -> String {
        parse(&input).iter().fold([50, 0], |acc, mv| {
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

    fn part2(&self, input: String) -> String {
        format!("this is part 2 and input was {}", input).to_string()
    }
}

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
