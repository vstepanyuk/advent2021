use clap::{App, AppSettings, Arg, SubCommand};

use solutions::*;

mod solutions;

macro_rules! solutions {
    ($( $mod_name:ident ), *) => {{
            let temp_vec: Vec<Box<dyn Solution>> = vec![
            $(
                Box::new($mod_name::DaySolution::new()),
            )*
            ];
            temp_vec
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut solutions = solutions!(
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14,
        day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
    );

    let matches = App::new("Advent of Code")
        .version("2021")
        .author("Vitali Stsepaniuk <contact@vitaliy.dev>")
        .about("Advent of Code 2021 Solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .required(true)
                .help("Day number")
                .validator(|v| match v.parse::<usize>().map_err(|e| e.to_string())? {
                    1..=25 => Ok(()),
                    _ => Err("must be between 1 and 25".into()),
                })
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Input filename")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("value")
                .short("v")
                .long("value")
                .help("Input value")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("part1").about("Get 1st solution"))
        .subcommand(SubCommand::with_name("part2").about("Get 2nd solution"))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    let day = matches.value_of("day").unwrap().parse::<usize>()?;
    let solution = solutions.get_mut(day - 1).unwrap();
    let file = matches.value_of("file");
    let value = matches.value_of("value");

    let input_value = match (file, value) {
        (Some(file), _) => std::fs::read_to_string(file).ok(),
        (_, Some(value)) => Some(value.to_string()),
        (_, _) => None,
    };

    match matches.subcommand_name() {
        Some("part1") => solution.part_1(input_value),
        Some("part2") => solution.part_2(input_value),
        _ => unreachable!(),
    }?;

    Ok(())
}
