use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::char::CharTryFromError;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    None,
}

impl Char {
    fn from(c: char) -> Char {
        match c {
            'X' => Char::X,
            'M' => Char::M,
            'A' => Char::A,
            'S' => Char::S,
            _ => Char::None,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    fn build_grid_and_starts<R: BufRead>(
        reader: R,
        target: Char,
    ) -> (Vec<(i32, i32)>, Vec<Vec<Char>>) {
        let mut starts = vec![];
        let grid: Vec<Vec<Char>> = reader
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let c = Char::from(c);
                        if c == target {
                            starts.push((i as i32, j as i32))
                        }
                        c
                    })
                    .collect()
            })
            .collect();
        (starts, grid)
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (xs, grid) = build_grid_and_starts(reader, Char::X);
        let mut xmasses = 0;
        for start in xs {
            for dir in vec![
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
            ] {
                xmasses += explore_x(&grid, start, dir)
            }
        }
        Ok(xmasses)
    }
    fn explore_x(grid: &Vec<Vec<Char>>, start: (i32, i32), dir: (i32, i32)) -> usize {
        let (mut i, mut j) = start;
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        for target in vec![Char::M, Char::A, Char::S] {
            (i, j) = (i + dir.0, j + dir.1);
            if i >= 0 && i < m && j >= 0 && j < n && grid[i as usize][j as usize] == target {
                continue;
            }
            return 0;
        }
        1
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (starts, grid) = build_grid_and_starts(reader, Char::A);
        let mut x_masses = 0;
        for start in starts {
            x_masses += explore_a(&grid, start)
        }
        Ok(x_masses)
    }
    fn explore_a(grid: &Vec<Vec<Char>>, start: (i32, i32)) -> usize {
        // a b
        //  X
        // d c
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let (a, b, c, d) = (
            (start.0 - 1, start.1 - 1),
            (start.0 - 1, start.1 + 1),
            (start.0 + 1, start.1 + 1),
            (start.0 + 1, start.1 - 1),
        );
        if a.0 < 0 || a.1 < 0 || b.0 < 0 || b.1 >= n || c.0 >= m || c.1 >= n || d.0 >= m || d.1 < 0
        {
            return 0;
        }
        // All the tuples are correct
        match grid[a.0 as usize][a.1 as usize] {
            Char::M => {
                if grid[c.0 as usize][c.1 as usize] != Char::S {
                    return 0;
                }
            }
            Char::S => {
                if grid[c.0 as usize][c.1 as usize] != Char::M {
                    return 0;
                }
            }
            _ => return 0,
        }
        match grid[b.0 as usize][b.1 as usize] {
            Char::M => {
                if grid[d.0 as usize][d.1 as usize] != Char::S {
                    return 0;
                }
            }
            Char::S => {
                if grid[d.0 as usize][d.1 as usize] != Char::M {
                    return 0;
                }
            }
            _ => return 0,
        }
        1
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
