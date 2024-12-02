use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

enum Direction {
    Incr,
    Decr,
}

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
        reader.lines().fold(vec![], |mut acc, line|{
            match line {
                Result::Ok(line) => {
                    let report: Vec<i32>   = line.split_whitespace().map(|e| e.parse().unwrap()).collect();
                    acc.push(report);
                    acc
                }
                Err(_) => panic!("failed line parse")
            }
        })
    }

    fn check_safety(report: &Vec<i32>) -> bool {
        let mut direction: Option<Direction> = None;
        for win in report.windows(2) {
            let delta = win[1] - win[0];
            if delta.abs() > 3 || delta.abs() == 0 {
                return false
            }
            match delta.signum() {
                -1 => match direction {
                    None => {
                        direction = Some(Direction::Decr);
                    }
                    Some(Direction::Incr) => {
                        return false
                    }
                    Some(Direction::Decr) => {}
                }
                1 => match direction {
                    None => {
                        direction = Some(Direction::Incr)
                    }
                    Some(Direction::Incr) => {}
                    Some(Direction::Decr) => {
                        return false
                    }
                }
                0 => {
                    return false
                }
                _ => panic!("invalid number")
            }
        }
        true
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let reports = parse_input(reader);
        let answer = reports.iter().filter(|&report| {
            check_safety(report)
        }).count();
        Ok(answer)
    }
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let reports = parse_input(reader);
        let answer = reports.iter().filter(|report| {
            if check_safety(report) {
                return true
            } else {
                for i in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(i);
                    if check_safety(&new_report) {
                        return true
                    }
                }
                return false
            }
        }).count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
