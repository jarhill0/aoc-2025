use std::num::ParseIntError;

pub fn int(string: &str) -> Result<i64, ParseIntError> {
    string.trim().parse()
}

pub fn tokens(line: &str) -> Vec<&str> {
    line.split_ascii_whitespace().collect()
}

pub fn ints(line: &str) -> Vec<i64> {
    tokens(line)
        .iter()
        .map(|token| int(token).unwrap())
        .collect()
}

pub fn token_lines(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(tokens).collect()
}
