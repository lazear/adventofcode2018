extern crate util;
use std::io;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Claim {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct ParseClaimError;

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let _claim = split.next().ok_or(ParseClaimError)?;
        let _at = split.next().ok_or(ParseClaimError)?;
        let coords: Vec<u32> = split
            .next()
            .ok_or(ParseClaimError)?
            .trim_matches(':')
            .split(',')
            .map(|s| s.parse::<u32>().map_err(|_| ParseClaimError))
            .collect::<Result<Vec<u32>, ParseClaimError>>()?;
        let dims = split
            .next()
            .ok_or(ParseClaimError)?
            .split('x')
            .map(|s| s.parse::<u32>().map_err(|_| ParseClaimError))
            .collect::<Result<Vec<u32>, ParseClaimError>>()?;
        Ok(Claim {
            x: coords[0],
            y: coords[1],
            w: dims[0],
            h: dims[1],
        })
    }
}

#[test]
fn claim_parse_test() {
    assert_eq!(
        "#123 @ 3,2: 5x4".parse::<Claim>(),
        Ok(Claim {
            x: 3,
            y: 2,
            w: 5,
            h: 4
        })
    );
}

fn part1(data: &[String]) -> Result<usize, ParseClaimError> {
    let claims = data
        .iter()
        .map(|x| x.parse::<Claim>())
        .collect::<Result<Vec<Claim>, ParseClaimError>>()?;
    // We know the whole fabric "is a very large sequare - at least 1000" on each side
    let max_x = claims.iter().fold(0, |acc, &c| acc.max(c.x + c.w));
    let max_y = claims.iter().fold(0, |acc, &c| acc.max(c.y + c.h));
    let mut grid = (0..max_x * max_y).map(|_| 0).collect::<Vec<u8>>();
    for c in &claims {
        for y in c.y..c.y + c.h {
            for x in c.x..c.x + c.w {
                grid[((y * max_x) + x) as usize] += 1;
            }
        }
    }
    Ok(grid.iter().filter(|&x| *x > 1).count())
}

fn part2(data: &[String]) -> Result<Option<usize>, ParseClaimError> {
    let claims = data
        .iter()
        .map(|x| x.parse::<Claim>())
        .collect::<Result<Vec<Claim>, ParseClaimError>>()?;
    // We know the whole fabric "is a very large sequare - at least 1000" on each side
    let max_x = claims.iter().fold(0, |acc, &c| acc.max(c.x + c.w));
    let max_y = claims.iter().fold(0, |acc, &c| acc.max(c.y + c.h));
    let mut grid = (0..max_x * max_y).map(|_| 0).collect::<Vec<u8>>();
    for c in &claims {
        for y in c.y..c.y + c.h {
            for x in c.x..c.x + c.w {
                grid[((y * max_x) + x) as usize] += 1;
            }
        }
    }
    for (i, c) in claims.iter().enumerate() {
        let mut m = true;
        for y in c.y..c.y + c.h {
            for x in c.x..c.x + c.w {
                if !m {
                    continue;
                }
                m = grid[((y * max_x) + x) as usize] == 1;
            }
        }
        if m {
            return Ok(Some(i + 1));
        }
    }
    Ok(None)
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), Ok(4));
}

#[test]
fn part2_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part2(&data), Ok(Some(3)));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
    Ok(())
}
