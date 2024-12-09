use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

type Calibration = (usize, Vec<usize>);

#[derive(Clone)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation
}

fn cartesian_product<T>(options: &[T], positions: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut result = vec![Vec::new(); 1];
    for _ in 0..positions {
        let mut new_result = vec![];
        for combination in &result {
            for option in options {
                let mut new_combination = combination.clone();
                new_combination.push(option.clone());
                new_result.push(new_combination)
            }
        }
        result = new_result;
    }
    result
}

fn is_valid1(c: &Calibration) -> bool {
    for operators in cartesian_product(
        &[Operator::Addition, Operator::Multiplication],
        c.1.len() - 1,
    ) {
        if compute(operators, c.1.clone()) == c.0 {
            return true;
        }
    }
    false
}

fn is_valid2(c: &Calibration) -> bool {
    for operators in cartesian_product(
        &[Operator::Addition, Operator::Multiplication, Operator::Concatenation],
        c.1.len() - 1,
    ) {
        if compute(operators, c.1.clone()) == c.0 {
            return true;
        }
    }
    false
}

fn compute(operators: Vec<Operator>, mut operands: Vec<usize>) -> usize {
    operands.reverse();
    operators
        .iter()
        .fold(operands.pop().unwrap(), |a, operator| {
            let b = operands.pop().unwrap();
            match operator {
                Operator::Addition => a + b,
                Operator::Multiplication => a * b,
                Operator::Concatenation => {
                    (a.to_string() + &b.to_string()).parse().unwrap()
                }
            }
        })
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Calibration> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            let (_, (target, _, operands)) =
                tuple((parse_num, tag(": "), parse_operands))(l.as_str()).unwrap();
            (target, operands) as Calibration
        })
        .collect()
}

fn parse_operands(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(' '), parse_num)(input)
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = parse_input(reader)
            .iter()
            .filter(|&c| is_valid1(c))
            .fold(0, |acc, c| acc + c.0);
        Ok(answer as usize)
    }
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = parse_input(reader)
            .iter()
            .filter(|&c| is_valid2(c))
            .fold(0, |acc, c| acc + c.0);
        Ok(answer)
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute(vec![Operator::Addition], vec![1, 2]), 3);
        assert_eq!(compute(vec![Operator::Multiplication], vec![1, 2]), 2);
        assert_eq!(compute(vec![Operator::Concatenation], vec![1, 2]), 12);
        assert_eq!(compute(vec![Operator::Concatenation], vec![1212344123, 212323]), 1212344123212323);
    }
}