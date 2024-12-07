use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{Cycle, Enumerate, Peekable};
use std::vec::IntoIter;

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
    Path(u8),
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

struct Guard {
    position: (i32, i32),
    dir: u8,
    _m: i32,
    _n: i32,
    _director: Peekable<Cycle<Enumerate<IntoIter<(i32, i32)>>>>,
}

impl Guard {
    fn new(position: (i32, i32), m: usize, n: usize) -> Guard {
        let mut director = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .peekable();
        Self{
            position: (position.0 as i32, position.1 as i32),
            dir: director.peek().unwrap().0 as u8,
            _m: m as i32,
            _n: n as i32,
            _director: director
        }
    }

    fn next(&mut self) -> Option<(usize, usize)> {
        let (_, dir) = self._director.peek().unwrap(); 
        let (i, j) = (self.position.0 + dir.0, self.position.1 + dir.1);
        if i < 0 || j < 0 || i >= self._m || j >= self._n {
            None
        } else {
            Some((i as usize, j as usize))
        }
    }
    
    fn turn(&mut self) {
        self._director.next();
        self.dir = (self._director.peek().unwrap().0 % 4) as u8
    }
    
    fn step(&mut self) {
        match self.next() {
            Some(pos) => self.position = (pos.0 as i32, pos.1 as i32),
            None => panic!("Attempted to step into the void")
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
        let (guard, mut grid) = parse_input(reader);
        let mut guard = Guard::new(guard, grid.len(), grid[0].len());
        grid[guard.position.0 as usize][guard.position.1 as usize] = Cell::Path(0);
        let mut path = 1;
        loop {
            let next = guard.next();
            match next {
                None => break,
                Some(next) => match grid[next.0][next.1] {
                    Cell::Obstacle => {
                        guard.turn();
                    },
                    Cell::Guard => panic!("WTF"),
                    Cell::Path(_) => {
                        guard.step();
                    },
                    Cell::Empty => {
                        path += 1;
                        grid[next.0][next.1] = Cell::Path(guard.dir);
                        guard.step();
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
    println!("\n=== Part 2 ===");

    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     // Navigate the route saving the direction of path. Cycle is possible if after placing an obstacle
    //     // paths will merge. That means that we either just crossed the path, or turning around will
    //     // meet the path.
    //     let (mut guard, mut grid) = parse_input(reader);
    //     let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    //
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
