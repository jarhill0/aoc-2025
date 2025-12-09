pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

pub fn look_up(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day1 {})),
        2 => Some(Box::new(day02::Day {})),
        3 => Some(Box::new(day03::Day {})),
        4 => Some(Box::new(day04::Day {})),
        5 => Some(Box::new(day05::Day {})),
        6 => Some(Box::new(day06::Day {})),
        7 => Some(Box::new(day07::Day {})),
        8 => Some(Box::new(day08::Day {})),
        9 => Some(Box::new(day09::Day {})),
        _ => None,
    }
}
