use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODO: Solve Part 1 of the puzzle
        let (mut first, mut second) = reader.lines().fold((vec![],vec![]), |(mut first, mut second), line| {
            match line {
                Result::Ok(line) => {
                    let nums: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
                    assert!(nums.len() == 2);
                    first.push(nums[0]);
                    second.push(nums[1]);
                    (first, second)
                }
                Result::Err(_) => {
                    panic!("failed to parse")
                }
            }
        });
        first.sort_unstable();
        second.sort_unstable();
        let answer = first.iter().zip(second.iter()).fold(0, |acc, (a,b)| {
            if a < b {
                acc + b-a
            } else {
                acc + a-b
            }
        });
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

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
