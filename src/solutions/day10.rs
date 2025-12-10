use crate::solutions::Solution;
use crate::util::int;
use std::collections::HashSet;

pub struct Day {}

fn parse(inp: &str) -> Vec<(HashSet<usize>, Vec<HashSet<usize>>)> {
    inp.lines()
        .map(|l| {
            let wds: Vec<&str> = l.split(' ').collect();
            let lights = wds[0][1..wds[0].len() - 1]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i)
                .collect();
            let schematics = wds[1..wds.len() - 1]
                .iter()
                .map(|w| {
                    w[1..w.len() - 1]
                        .split(',')
                        .map(|x| int(x).unwrap() as usize)
                        .collect()
                })
                .collect();
            (lights, schematics)
        })
        .collect()
}

fn fewest_presses(problem: &(HashSet<usize>, Vec<HashSet<usize>>)) -> i64 {
    let (desired_lights, schematics) = problem;
    let ways = 1 << schematics.len();
    (0..ways)
        .map(bits_enabled)
        .filter(|using_schematics| desired_lights == &light_state(schematics, using_schematics))
        .map(|s| s.len() as i64)
        .min()
        .unwrap()
}

fn light_state(schematics: &Vec<HashSet<usize>>, using: &HashSet<usize>) -> HashSet<usize> {
    let mut set = HashSet::new();
    using.iter().for_each(|i| {
        schematics[*i].iter().for_each(|l| {
            if set.contains(l) {
                set.remove(l);
            } else {
                set.insert(*l);
            }
        })
    });
    set
}

fn bits_enabled(bits: i64) -> HashSet<usize> {
    let mut bits = bits;
    let mut enabled = HashSet::new();
    let mut light_ind = 0;
    while bits != 0 {
        if bits & 1 == 1 {
            enabled.insert(light_ind);
        }

        bits >>= 1;
        light_ind += 1;
    }
    enabled
}

impl Solution for Day {
    fn part1(&self, input: &str) -> String {
        let problems = parse(input);
        let ans: i64 = problems.iter().map(fewest_presses).sum();
        format!("{}", ans)
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn example() {
        let d = Day {};

        let result1 = d.part1(EXAMPLE_INPUT);
        assert_eq!(result1, "7");

        let result2 = d.part2(EXAMPLE_INPUT);
        assert_eq!(result2, "");
    }

    #[test]
    fn actual() {
        let d = Day {};
        let input = crate::input(10);

        let result1 = d.part1(&input);
        assert_eq!(result1, "417");

        let result2 = d.part2(&input);
        assert_eq!(result2, "");
    }
}
