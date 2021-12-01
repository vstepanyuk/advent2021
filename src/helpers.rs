use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: Option<String>) -> Vec<T> {
    input
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}
