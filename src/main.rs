use std::env::args;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

mod solutions;
mod util;

fn main() {
    let mut args = args();
    if args.len() == 3 {
        let day_num = args.nth(1).unwrap().parse::<u8>();
        let part_num = args.next().unwrap().parse::<u8>(); // index 2

        let solution = match (day_num, part_num) {
            (Ok(day_num), Ok(part_num)) => match solutions::look_up(day_num) {
                Some(solver) => match part_num {
                    1 => solver.part1(input(day_num)),
                    2 => solver.part2(input(day_num)),
                    _ => quit("Bad partnum. Should be 1 or 2."),
                },
                None => quit("Unknown daynum."),
            },
            _ => quit("Error parsing daynum and/or partnum as int."),
        };

        println!("{}", solution)
    } else {
        quit(&format!(
            "Usage: {} <daynum> <partnum>",
            args.next().unwrap_or("aoc-2025".to_string())
        ));
    }
}

fn quit(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn input(day_num: u8) -> String {
    let inp = if let Some(i) = load_local_input(day_num) {
        i
    } else {
        match download_input(day_num) {
            Ok(i) => i,
            Err(message) => {
                quit(&format!("Error downloading input: {}", message));
            }
        }
    };
    inp.trim().to_string()
}

// TODO: all paths should probably be relative to the source location rather than the pwd…
fn local_input_path(day_num: u8) -> PathBuf {
    Path::new(&format!("./inputs/{}.txt", day_num)).to_path_buf()
}

fn load_local_input(day_num: u8) -> Option<String> {
    std::fs::read_to_string(local_input_path(day_num)).ok()
}

fn aoc_token_path() -> PathBuf {
    Path::new("./.aoc-token").to_path_buf()
}

fn load_access_token() -> Result<String, std::io::Error> {
    std::fs::read_to_string(aoc_token_path())
}
fn download_input(day_num: u8) -> Result<String, reqwest::Error> {
    let mut token = String::new();
    if let Ok(t) = load_access_token() {
        token = t;
    } else {
        print!("Enter session cookie: ");
        stdout().flush().expect("flushing stdout failed");

        stdin()
            .read_line(&mut token)
            .expect("reading token from stdin failed");

        token = token.trim().to_string();

        std::fs::write(aoc_token_path(), &token).expect("writing token file failed");
    }

    let client = reqwest::blocking::Client::new();
    let contents = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            2025, day_num,
        ))
        .header("Cookie", format!("session={}", token))
        .header(
            "User-Agent",
            format!("AoC 2025 Joey Rees-Hill <aoc@{}.net> v0.1", "reeshill"),
        )
        .send()?
        .text()?;

    std::fs::create_dir_all(
        local_input_path(day_num)
            .parent()
            .expect("expected input file to be in directory"),
    )
    .expect("creating input directory failed");
    std::fs::write(local_input_path(day_num), &contents).expect("writing input file failed");

    Ok(contents)
}
