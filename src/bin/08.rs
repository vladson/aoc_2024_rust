use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(PartialEq)]
enum Plot {
    Empty,
    Antinode,
    Node(char),
}

struct Antennae {
    _freq: char,
    i: usize,
    j: usize,
}

impl Antennae {
    fn first_antinode<F>(
        a1: &Antennae,
        a2: &Antennae,
        bc: F
    ) -> Vec<Option<(usize, usize)>> 
    where F: Fn((i32, i32)) -> bool {
        let mut res = Vec::with_capacity(2);
        let i = a1.i as i32 + (a1.i as i32 - a2.i as i32);
        let j = a1.j as i32 + (a1.j as i32 - a2.j as i32);
        if bc((i, j)) {
            res.push(Some((i as usize, j as usize)));
        } else {
            res.push(None);
        }
        let i = a2.i as i32 + (a2.i as i32 - a1.i as i32);
        let j = a2.j as i32 + (a2.j as i32 - a1.j as i32);
        if bc((i, j)) {
            res.push(Some((i as usize, j as usize)));
        } else {
            res.push(None);
        }
        res
    }

    fn resonant_antinodes<F>(a1: &Antennae, a2: &Antennae, bc: F) -> Vec<Option<(usize, usize)>>
    where
        F: Fn((i32, i32)) -> bool,
    {
        let mut res = Vec::new();
        let d1 = (a2.i as i32 - a1.i as i32, a2.j as i32 - a1.j as i32);
        let mut node = (a1.i as i32 + d1.0, a1.j as i32 + d1.1);
        while bc(node) {
            res.push(Some((node.0 as usize, node.1 as usize)));
            node = (node.0 + d1.0, node.1 + d1.1);
        }
        let d2 = (a1.i as i32 - a2.i as i32, a1.j as i32 - a2.j as i32);
        let mut node = (a2.i as i32 + d2.0, a2.j as i32 + d2.1);
        while bc(node) {
            res.push(Some((node.0 as usize, node.1 as usize)));
            node = (node.0 + d2.0, node.1 + d2.1);
        }
        res
    }
}

impl Antennae {
    fn new(ch: char, i: usize, j: usize) -> Antennae {
        Self {
            _freq: ch,
            i: i,
            j: j,
        }
    }
}

fn print_land(p0: &Vec<Vec<Plot>>) {
    for row in p0 {
        for plot in row {
            match plot {
                Plot::Empty => print!("."),
                Plot::Antinode => print!("#"),
                Plot::Node(f) => print!("{}", f),
            }
        }
        println!();
    }
}

fn parse_input<R: BufRead>(reader: R) -> ([Vec<Antennae>; 256], Vec<Vec<Plot>>) {
    let mut frequencies: [Vec<Antennae>; 256] = std::array::from_fn(|_| Vec::new());
    let layofland = reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == '.' {
                        Plot::Empty
                    } else {
                        if !ch.is_ascii() {
                            panic!("non ASCII frequency");
                        }
                        frequencies[ch as usize].push(Antennae::new(ch, i, j));
                        Plot::Node(ch)
                    }
                })
                .collect()
        })
        .collect();
    (frequencies, layofland)
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (frequencies, mut layofland) = parse_input(reader);
        let (m, n) = (layofland.len(), layofland[0].len());
        let bound_checker = |node: (i32, i32)| {
            !(node.0 < 0 || node.1 < 0 || node.0 >= m as i32 || node.1 >= n as i32)
        };
        let ans = frequencies
            .iter()
            .filter(|&f| !f.is_empty())
            .fold(0, |mut acc, freq| {
                for i in 0..freq.len() {
                    for j in i + 1..freq.len() {
                        for point in Antennae::first_antinode(&freq[i], &freq[j], &bound_checker) {
                            match point {
                                None => {}
                                Some(point) => match layofland[point.0][point.1] {
                                    Plot::Empty | Plot::Node(_) => {
                                        layofland[point.0][point.1] = Plot::Antinode;
                                        acc += 1
                                    }
                                    _ => continue,
                                },
                            }
                        }
                    }
                }
                acc
            });
        print_land(&layofland);
        Ok(ans)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (frequencies, mut layofland) = parse_input(reader);
        let (m, n) = (layofland.len(), layofland[0].len());
        let bound_checker = |node: (i32, i32)| {
            !(node.0 < 0 || node.1 < 0 || node.0 >= m as i32 || node.1 >= n as i32)
        };
        let ans = frequencies
            .iter()
            .filter(|&f| !f.is_empty())
            .fold(0, |mut acc, freq| {
                for i in 0..freq.len() {
                    for j in i + 1..freq.len() {
                        for point in
                            Antennae::resonant_antinodes(&freq[i], &freq[j], &bound_checker)
                        {
                            match point {
                                None => {}
                                Some(point) => match layofland[point.0][point.1] {
                                    Plot::Empty | Plot::Node(_) => {
                                        layofland[point.0][point.1] = Plot::Antinode;
                                        acc += 1
                                    }
                                    _ => continue,
                                },
                            }
                        }
                    }
                }
                acc
            });
        print_land(&layofland);
        Ok(ans)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
