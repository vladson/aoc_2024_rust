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

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Obstacle,
    Empty,
    Path(u8),
    Guard,
    Change
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

#[derive(Clone, Debug)]
struct Guard {
    position: (i32, i32),
    dir: u8,
    _m: i32,
    _n: i32,
    _director: Peekable<Cycle<Enumerate<IntoIter<(i32, i32)>>>>,
}

impl Guard {
    fn new(position: (usize, usize), m: usize, n: usize) -> Guard {
        let mut director = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            .enumerate()
            .cycle()
            .peekable();
        Self {
            position: (position.0 as i32, position.1 as i32),
            dir: director.peek().unwrap().0 as u8,
            _m: m as i32,
            _n: n as i32,
            _director: director,
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
            None => panic!("Attempted to step into the void"),
        }
    }
}

fn parse_input<R: BufRead>(reader: R) -> ((usize, usize), Vec<Vec<Cell>>) {
    let mut guard = (0, 0);
    let grid = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    let c = Cell::from(c);
                    if c == Cell::Guard {
                        guard = (i, j)
                    }
                    c
                })
                .collect()
        })
        .collect();
    (guard, grid)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (start, mut grid) = parse_input(reader);
        let mut guard = Guard::new(start, grid.len(), grid[0].len());
        grid[start.0][start.1] = Cell::Path(0);
        let mut path = 1;
        loop {
            let next = guard.next();
            match next {
                None => break,
                Some(next) => match grid[next.0][next.1] {
                    Cell::Obstacle => {
                        guard.turn();
                    }
                    Cell::Guard => panic!("WTF"),
                    Cell::Path(_) => {
                        guard.step();
                    }
                    Cell::Empty => {
                        path += 1;
                        grid[next.0][next.1] = Cell::Path(guard.dir);
                        guard.step();
                    },
                    Cell::Change => panic!("wtf")
                },
            }
        }
        grid[start.0][start.1] = Cell::Guard;
        Ok(path)
    }
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Navigate the route saving the direction of path. Cycle is possible if after placing an obstacle
        // paths will merge. That means that we either just crossed the path, or turning around will
        // meet the path.
        let (start, mut grid) = parse_input(reader);
        let mut guard = Guard::new(start, grid.len(), grid[0].len());
        grid[start.0][start.1] = Cell::Path(0);
        let mut options = 0;
        let mut turned = false;
        // walk the path.
        loop {
            let next = guard.next();
            match next {
                None => break,
                Some(next) => match grid[next.0][next.1] {
                    Cell::Obstacle => {
                        guard.turn();
                        turned = true;
                    }
                    Cell::Guard => panic!("WTF"),
                    Cell::Path(dir) => {
                        guard.step();
                    }
                    Cell::Empty => {
                        if turned && attempt(grid.clone(), guard.clone()) {
                            options += 1
                        }
                        grid[next.0][next.1] = Cell::Path(guard.dir);
                        guard.step();
                    },
                    Cell::Change => panic!("wtf")
                },
            }
        }
        Ok(options)
    }
    fn attempt(mut grid: Vec<Vec<Cell>>, mut guard: Guard) -> bool {
        // we are placing an obstacle there. Print the grid and guard
        let _start = guard.position;
        let _dir = guard.dir;
        let next = guard.next().unwrap();
        grid[next.0][next.1] = Cell::Obstacle;
        loop {
            let next = guard.next();
            match next {
                None => return false,
                Some(next) => match grid[next.0][next.1] {
                    Cell::Obstacle => {
                        guard.turn();
                    }
                    Cell::Guard => panic!("WTF"),
                    Cell::Path(dir) => {
                        // we are crossing the path
                        if dir == guard.dir{
                            // println!("=== Attempt ===");
                            // println!("start: {:?}, dir: {}, next: {:?}", _start, _dir, next);
                            // print_grid(grid, _start, next);
                            return true
                        }
                        guard.step();
                    }
                    Cell::Empty => {
                        grid[next.0][next.1] = Cell::Path(guard.dir);
                        guard.step();
                    },
                    Cell::Change => panic!("wtf")
                },
            }
        }
    }

    fn print_grid(mut grid: Vec<Vec<Cell>>, start: (i32, i32), obst: (usize, usize)) {
        grid[start.0 as usize][start.1 as usize] = Cell::Guard;
        grid[obst.0][obst.1] = Cell::Change;
        for row in grid.iter() {
            for cell in row.iter() {
                print!(
                    "{}",
                    match cell {
                        Cell::Empty => '.',
                        Cell::Obstacle => '#',
                        Cell::Path(dir) => {
                            match dir {
                                0 => '↑',
                                1 => '→',
                                2 => '↓',
                                3 => '←',
                                _ => panic!("WTF"),
                            }
                        },
                        Cell::Guard => '^',
                        Cell::Change => 'O',
                    }
                )
            }
            println!()
        }

    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
