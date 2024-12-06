use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

type Pair = (u32, u32);
type Update = Vec<u32>;

#[derive(Debug)]
struct Rule {
    indegree: u32,
    constraints: Vec<u32>,
}

impl Rule {
    fn from_constraint(constraint: u32) -> Rule {
        Self {
            indegree: 0,
            constraints: vec![constraint],
        }
    }

    fn starting_1() -> Rule {
        Self {
            indegree: 1,
            constraints: vec![],
        }
    }
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<Pair>, Vec<Update>) {
    let mut rules: Vec<Pair> = vec![];
    let mut updates: Vec<Update> = vec![];
    let mut prima = true;
    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            if prima {
                prima = false;
                continue;
            } else {
                break;
            }
        }
        if prima {
            match parse_pair(line.as_str()) {
                IResult::Ok((_, pair)) => rules.push(pair),
                _ => panic!("WTF++"),
            }
        } else {
            match parse_update(line.as_str()) {
                IResult::Ok((_, pair)) => updates.push(pair),
                _ => panic!("WTF++"),
            }
        }
    }
    (rules, updates)
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    separated_list1(char(','), parse_number)(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}
fn build_rules(pairs: Vec<Pair>) -> HashMap<u32, Rule> {
    let mut page_rules = HashMap::new();
    for (a, b) in pairs {
        page_rules
            .entry(a)
            .and_modify(|r: &mut Rule| r.constraints.push(b))
            .or_insert(Rule::from_constraint(b));
        page_rules
            .entry(b)
            .and_modify(|r| (*r).indegree += 1)
            .or_insert(Rule::starting_1());
    }
    page_rules
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(parse_number, char('|'), parse_number)(input)
}
fn main() -> Result<()> {

    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (pairs, updates) = parse_input(reader);
        let mut page_rules = build_rules(pairs);
        let ans: u32 = updates
            .iter()
            .filter(|update| {
                let mut upto = vec![];
                for u in update.iter() {
                    let broken = page_rules
                        .entry(*u)
                        .or_insert(Rule::starting_1())
                        .constraints
                        .iter()
                        .filter(|c| upto.contains(c))
                        .count()
                        > 0;
                    if broken {
                        return false;
                    }
                    upto.push(u);
                }
                true
            })
            .map(|u| u[u.len() / 2])
            .sum();
        Ok(ans as usize)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (pairs, updates) = parse_input(reader);
        let mut page_rules = build_rules(pairs);
        let bad: Vec<Update> = updates
            .into_iter()
            .filter(|update| {
                let mut upto = vec![];
                for u in update.iter() {
                    let broken = page_rules
                        .entry(*u)
                        .or_insert(Rule::starting_1())
                        .constraints
                        .iter()
                        .filter(|c| upto.contains(c))
                        .count()
                        > 0;
                    if broken {
                        return true;
                    }
                    upto.push(u);
                }
                false
            }).collect();
        let ans: u32 = bad.into_iter().map(|mut update| {
            update.sort_by(|a, b| {
                let a_blocked = page_rules.entry(*a)
                    .or_insert(Rule::starting_1())
                    .constraints.contains(b);
                let b_blocked = page_rules.entry(*b)
                    .or_insert(Rule::starting_1())
                    .constraints.contains(a);
                match (a_blocked, b_blocked) {
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    (false, false) => std::cmp::Ordering::Equal,
                    _ => panic!("how?")
                }
            });
            update
        }).map(|u| u[u.len()/2]).sum();
        Ok(ans as usize)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
