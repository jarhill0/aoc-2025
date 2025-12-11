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

#[derive(Copy, Clone)]
struct PathCount {
    neither: i64,
    both: i64,
    fft_only: i64,
    dac_only: i64,
}

impl PathCount {
    fn account_for(&self, device_name: &str) -> PathCount {
        match device_name {
            "fft" => PathCount {
                neither: 0,
                dac_only: 0,
                fft_only: self.fft_only + self.neither,
                both: self.both + self.dac_only,
            },
            "dac" => PathCount {
                neither: 0,
                fft_only: 0,
                dac_only: self.dac_only + self.neither,
                both: self.both + self.dac_only,
            },
            _ => *self,
        }
    }
}

fn add_path_counts(a: PathCount, b: PathCount) -> PathCount {
    PathCount {
        neither: a.neither + b.neither,
        both: a.both + b.both,
        fft_only: a.fft_only + b.fft_only,
        dac_only: a.dac_only + b.dac_only,
    }
}

fn count_ways_fft_dac(
    links: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, PathCount>,
    to: &str,
) -> PathCount {
    // as if "from" out "to" you, to accommodate the way I chose to parse the input
    if to == "out" {
        return PathCount {
            neither: 1,
            both: 0,
            fft_only: 0,
            dac_only: 0,
        };
    }
    if memo.contains_key(to) {
        return memo[to];
    }

    let ways = links[to]
        .iter()
        .map(|next_device| count_ways_fft_dac(links, memo, next_device))
        .reduce(add_path_counts)
        .unwrap();
    let ways = ways.account_for(to);
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
        let links = parse(input);
        let mut memo = HashMap::new();
        format!("{}", count_ways_fft_dac(&links, &mut memo, "svr").both)
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
    const EXAMPLE_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "5");

        let result2 = d.part2(EXAMPLE_INPUT_2);
        assert_eq!(result2, "2");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(11);

        let result1 = d.part1(&input);
        assert_eq!(result1, "753");

        let result2 = d.part2(&input);
        assert_eq!(result2, "450854305019580");
    }
}
