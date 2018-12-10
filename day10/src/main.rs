use std::io;
use std::num::ParseIntError;
use std::str::FromStr;
use util;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
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
            .split(|c| c == ',' || c == '<' || c == '>')
            .map(str::trim)
            .filter(|s| s.starts_with(|c: char| c.is_numeric() || c == '-'))
            .map(str::parse::<i64>)
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .map_err(|_| CoordError::ParseIntError)?;
        Ok(Coord {
            x: *n.get(0).ok_or(CoordError::InvalidData)?,
            y: *n.get(1).ok_or(CoordError::InvalidData)?,
            vx: *n.get(2).ok_or(CoordError::InvalidData)?,
            vy: *n.get(3).ok_or(CoordError::InvalidData)?,
        })
    }
}

fn part1(data: &[String]) -> Result<(), CoordError> {
    let mut coords = data
        .iter()
        .map(|s| s.parse::<Coord>())
        .collect::<Result<Vec<Coord>, CoordError>>()?;
    let mut ticks = 0;
    loop {
        let y_min = coords
            .iter()
            .map(|c| c.y)
            .min()
            .ok_or(CoordError::InvalidData)?;
        let x_min = coords
            .iter()
            .map(|c| c.x)
            .min()
            .ok_or(CoordError::InvalidData)?;
        let y_max = coords
            .iter()
            .map(|c| c.y)
            .max()
            .ok_or(CoordError::InvalidData)?;
        let y_stride = y_max - y_min;
        if y_stride < 20 {
            println!("{} ticks", ticks);
            for y in 0..=y_stride {
                let mut array = ['.'; 150];
                coords
                    .iter()
                    .filter(|c| c.y - y_min == y)
                    .map(|c| (c.x - x_min) as usize)
                    .filter(|&i| i <= 150)
                    .for_each(|i| array[i] = '#');

                println!("{:?}", array.iter().collect::<String>());
            }
        }

        coords.iter_mut().for_each(|c| {
            c.x += c.vx;
            c.y += c.vy;
        });
        ticks += 1;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("{:?}", part1(&data));
    Ok(())
}
