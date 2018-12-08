use std::collections::VecDeque;
use std::io;
use std::num::ParseIntError;
use util;

fn part1_recursive(data: &mut VecDeque<usize>) -> Option<usize> {
    let mut m = 0;
    let c = data.pop_front()?;
    let e = data.pop_front()?;
    for _ in 0..c {
        m += part1_recursive(data)?;
    }
    for _ in 0..e {
        m += data.pop_front()?;
    }
    Some(m)
}

fn part1(data: &str) -> Result<Option<usize>, ParseIntError> {
    Ok(part1_recursive(
        &mut data
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<VecDeque<usize>, ParseIntError>>()?,
    ))
}

fn part2_recursive(data: &mut VecDeque<usize>) -> Option<usize> {
    let mut m = 0;
    let c = data.pop_front()?;
    let e = data.pop_front()?;
    if c > 0 {
        let children = (0..c)
            .map(|_| part2_recursive(data))
            .collect::<Option<Vec<usize>>>()?;
        for _ in 0..e {
            let idx = data.pop_front()?;
            if let Some(val) = children.get(idx - 1) {
                m += val;
            }
        }
    } else {
        for _ in 0..e {
            m += data.pop_front()?;
        }
    }
    Some(m)
}

fn part2(data: &str) -> Result<Option<usize>, ParseIntError> {
    Ok(part2_recursive(
        &mut data
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<VecDeque<usize>, ParseIntError>>()?,
    ))
}

#[test]
fn part1_test() {
    assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), Ok(Some(138)));
}

#[test]
fn part2_test() {
    assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), Ok(Some(66)));
}

fn main() -> io::Result<()> {
    let data = util::read("input.txt")?;
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
    Ok(())
}
