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

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

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

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let reports = parse_input(reader);
        let answer = reports.iter().filter(|report| {
            let decr = if report[1] < report[0] {true} else {false};
            for i in 1..report.len() {
                if decr {
                    if report[i] > report[i-1] {
                        return false
                    }
                    let delta = report[i-1] - report[i];
                    if delta == 0 || delta > 3 {
                        return false
                    }
                } else {
                    if report[i] < report[i-1] {
                        return false
                    }
                    let delta = report[i] - report[i-1];
                    if delta == 0 || delta > 3 {
                        return false
                    }
                }
            }
            true
        }).count();
        Ok(answer)
    }
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
