use std::io;
use std::num::ParseIntError;
use std::collections::VecDeque;
use util;

fn part1(players: usize, value: usize) -> Option<usize> {
    let mut circle = VecDeque::from(vec![0usize]);
    let mut scores = (0..players).map(|_| 0).collect::<Vec<usize>>();
    for m in 1..=value {
        if m % 23 == 0 {
            (0..7).for_each(|_| {
                let x = circle.pop_back().unwrap();
                circle.push_front(x);
            });
            scores[m % players] += m + circle.pop_front()?;
        } else {
            (0..2).for_each(|_| {
                let x = circle.pop_front().unwrap();
                circle.push_back(x);
            });
            circle.push_front(m);
        }
    }
    scores.into_iter().max()
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    for line in &data {
        let line = line
            .split_whitespace()
            .filter(|s| s.starts_with(char::is_numeric))
            .map(str::parse::<usize>)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .unwrap();
        assert_eq!(part1(line[0], line[1]), Some(line[2]));
    }
}

fn main() -> io::Result<()> {
    let data = util::read("input.txt")?
        .split_whitespace()
        .filter(|s| s.starts_with(char::is_numeric))
        .map(str::parse::<usize>)
        .collect::<Result<Vec<usize>, ParseIntError>>()
        .unwrap();
    println!("Part 1: {:?}", part1(data[0], data[1]));
    println!("Part 2: {:?}", part1(data[0], data[1]*100));
    Ok(())
}
