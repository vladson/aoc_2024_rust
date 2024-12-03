use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::ErrorKind::ConnectionAborted;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

enum Command {
    Activate,
    Deactivate,
    Result(i32),
}

impl Command {
    fn from(input: String) -> Command {
        let re_inner = Regex::new("^mul\\((\\d{1,3}),(\\d{1,3})\\)$").unwrap();
        match input.as_str() {
            "do()" => Command::Activate,
            "don't()" => Command::Deactivate,
            _ => {
                let mut res = 0;
                for (_, [a1, a2]) in re_inner
                    .captures_iter(input.as_str())
                    .map(|caps| caps.extract())
                {
                    res = a1.parse::<i32>().unwrap() * a2.parse::<i32>().unwrap()
                }
                Command::Result(res)
            }
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
        let answer: i32 = reader
            .lines()
            .flat_map(|line| match line {
                Result::Ok(line) => {
                    let mut parts = vec![];
                    for (_, [a1, a2]) in re.captures_iter(line.as_str()).map(|caps| caps.extract())
                    {
                        parts.push(a1.parse::<i32>().unwrap() * a2.parse::<i32>().unwrap())
                    }
                    parts
                }
                Err(_) => panic!("invalid string"),
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let re_outer = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)|do\\(\\)|don't\\(\\)").unwrap();
        let commands: Vec<String> = reader
            .lines()
            .flat_map(|line| {
                match line {
                    Result::Ok(line) => {
                        let matches: Vec<String> = re_outer
                            .find_iter(&line) // Borrow the line
                            .map(|m| m.as_str().to_string()) // Convert matches to owned Strings
                            .collect();
                        matches.into_iter()
                    }
                    Err(_) => panic!("invalid string"),
                }
            })
            .collect();
        let answer = commands.iter().map(|c| Command::from(c.to_string())).fold(
            (0, true),
            |(acc, cap), command| match command {
                Command::Activate => (acc, true),
                Command::Deactivate => (acc, false),
                Command::Result(res) => {
                    if cap {
                        (acc + res, true)
                    } else {
                        (acc, false)
                    }
                }
            },
        );
        Ok(answer.0)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
