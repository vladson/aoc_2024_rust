use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(PartialEq, Debug)]
enum Cell {
    Obstacle,
    Empty,
    Path,
    Guard,
}

impl Cell {
    fn from(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Obstacle,
            '^' => Cell::Guard,
            _ => {
                panic!("unknown")
            }
        }
    }
}

fn parse_input<R: BufRead>(reader: R) -> ((i32, i32), Vec<Vec<Cell>>) {
    let mut guard = (0, 0);
    let grid = reader
            .lines()
            .map_while(Result::ok)
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(i, line)| {
                line.chars().enumerate().map(|(j, c)| {
                    let c = Cell::from(c);
                    if c == Cell::Guard {
                        guard = (i as i32, j as i32)
                    }
                    c
                }).collect()
            }).collect();
    (guard, grid)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut guard, mut grid) = parse_input(reader);
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut direction = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .cycle()
            .peekable();
        fn step(pos: (i32, i32), dir: &(i32, i32), m: i32, n: i32) -> Option<(i32, i32)> {
            let (i, j) = (pos.0 + dir.0, pos.1 + dir.1);
            if i < 0 || j < 0 || i >= m || j >= n {
                None
            } else {
                Some((i, j))
            }
        }
        grid[guard.0 as usize][guard.1 as usize] = Cell::Path;
        let mut path = 1;
        loop {
            let next = step(guard, direction.peek().unwrap(), m, n);
            match next {
                None => break,
                Some(next) => match grid[next.0 as usize][next.1 as usize] {
                    Cell::Obstacle => {
                        direction.next();
                    },
                    Cell::Guard => panic!("WTF"),
                    Cell::Path => {
                        guard = next;
                    },
                    Cell::Empty => {
                        path += 1;
                        grid[next.0 as usize][next.1 as usize] = Cell::Path;
                        guard = next;
                    }
                },
            }
        }
        Ok(path)
    }
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
