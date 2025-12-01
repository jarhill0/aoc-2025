pub trait Solution {
    fn part1(&self, input: String) -> String;
    fn part2(&self, input: String) -> String;
}

mod day01;

pub fn look_up(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day1 {})),
        _ => None,
    }
}
