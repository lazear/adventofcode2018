extern crate util;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum CoordError {
    ParseIntError,
    InvalidData,
}

impl FromStr for Coord {
    type Err = CoordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s
            .split(", ")
            .map(str::parse::<i32>)
            .collect::<Result<Vec<i32>, ParseIntError>>()
            .map_err(|_| CoordError::ParseIntError)?;
        Ok(Coord {
            x: *n.get(0).ok_or(CoordError::InvalidData)?,
            y: *n.get(1).ok_or(CoordError::InvalidData)?,
        })
    }
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(data: &[String]) -> Result<usize, CoordError> {
    let coords = data
        .iter()
        .map(|s| s.parse::<Coord>())
        .collect::<Result<Vec<Coord>, CoordError>>()?;

    // Define the size of our search space
    let max = coords.iter().fold(Coord { x: 0, y: 0 }, |acc, &c| Coord {
        x: acc.x.max(c.x),
        y: acc.y.max(c.y),
    });
    let min = coords
        .iter()
        .fold(Coord { x: max.x, y: max.y }, |acc, &c| Coord {
            x: acc.x.min(c.x),
            y: acc.y.min(c.y),
        });
    let mut infinite = (0..coords.len()).map(|_| true).collect::<Vec<bool>>();
    let mut scores = (0..coords.len()).map(|_| 0).collect::<Vec<i32>>();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let c = Coord { x, y };
            let s = coords.iter().map(|x| c.distance(x)).collect::<Vec<i32>>();
            let m = s.iter().min().ok_or(CoordError::InvalidData)?;
            let idx = s
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if x == m { Some(i) } else { None })
                .collect::<Vec<usize>>();

            if idx.len() == 1 {
                scores[idx[0]] += 1;
                if x == min.x || x == max.x || y == min.y || y == max.y {
                    infinite[idx[0]] = false;
                }
            } else if idx.len() == 0 {
                return Err(CoordError::InvalidData);
            }
        }
    }
    Ok(*scores
        .iter()
        .zip(infinite.iter())
        .filter(|(_, &c)| c)
        .map(|(s, _)| s)
        .max()
        .ok_or(CoordError::InvalidData)? as usize)
}

fn part2(data: &[String], cutoff: i32) -> Result<usize, CoordError> {
    let coords = data
        .iter()
        .map(|s| s.parse::<Coord>())
        .collect::<Result<Vec<Coord>, CoordError>>()?;

    // Define the size of our search space
    let max = coords.iter().fold(Coord { x: 0, y: 0 }, |acc, &c| Coord {
        x: acc.x.max(c.x),
        y: acc.y.max(c.y),
    });
    let min = coords
        .iter()
        .fold(Coord { x: max.x, y: max.y }, |acc, &c| Coord {
            x: acc.x.min(c.x),
            y: acc.y.min(c.y),
        });

    let mut safe_region = Vec::new();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let c = Coord { x, y };
            let s = coords.iter().map(|x| c.distance(x)).collect::<Vec<i32>>();
            if s.iter().sum::<i32>() < cutoff {
                safe_region.push(c);
            }
        }
    }
    Ok(safe_region.len())
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), Ok(17));
}

#[test]
fn part2_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part2(&data, 32), Ok(16));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data, 10_000));
    Ok(())
}
