use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[derive(PartialEq)]
enum Char {
    X,
    M,
    A,
    S,
    None
}

impl Char {
    fn from(c: char) -> Char {
        match c {
            'X' => Char::X,
            'M' => Char::M,
            'A' => Char::A,
            'S' => Char::S,
            _   => Char::None
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut xs = vec![];
        let grid: Vec<Vec<Char>> = reader
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .map( |(i, line)| {
                line.chars().enumerate().map( |(j, c)| {
                    let c = Char::from(c);
                    if c == Char::X {
                        xs.push((i as i32, j as i32))
                    }
                    c
                }).collect()
            })
            .collect();
        let mut xmasses = 0;
        for start in xs {
            for dir in vec![(-1,0), (-1, 1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)] {
                xmasses += explore(&grid, start, dir)
            }
        }
        Ok(xmasses)
    }
    fn explore(grid: &Vec<Vec<Char>>, start: (i32, i32), dir: (i32, i32)) -> usize {
        let (mut i, mut j) = start;
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        for target in vec![Char::M, Char::A, Char::S] {
            (i, j) = (i+dir.0, j + dir.1);
            if i >= 0 && i < m && j >= 0 && j < n && grid[i as usize][j as usize] == target {
                continue
            }
            return 0
        }
        1
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
