use crate::solutions::Solution;
use crate::util::tokens;
use std::collections::HashMap;

pub struct Day {}

fn parse(inp: &str) -> HashMap<String, Vec<String>> {
    inp.lines()
        .map(|l| {
            let (key, vals) = l.split_once(": ").unwrap();
            (
                key.to_string(),
                tokens(vals)
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

fn count_ways(
    links: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
    to: &str,
) -> i64 {
    // as if "from" out "to" you, to accommodate the way I chose to parse the input
    if to == "out" {
        return 1;
    }
    if memo.contains_key(to) {
        return memo[to];
    }

    let ways = links[to]
        .iter()
        .map(|next_device| count_ways(links, memo, next_device))
        .sum();
    memo.insert(to.to_string(), ways);

    ways
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let links = parse(input);
        let mut memo = HashMap::new();
        format!("{}", count_ways(&links, &mut memo, "you"))
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "5");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(11);

        let result1 = d.part1(&input);
        assert_eq!(result1, "753");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
