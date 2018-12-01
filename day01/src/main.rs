use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::collections::HashSet;

fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn part1(data: &Vec<i64>) -> i64 {
    data.iter().fold(0, |acc, &x| acc + x)
}

fn part2(data: &Vec<i64>) -> i64 {
    let mut set = HashSet::<i64>::new();
    let mut r = 0;
    loop {
        for &x in data {
            if set.contains(&r) {
                return r
            } else {                
                set.insert(r);
                r += x;
            }
        }
    }
}

#[test]
fn part2_test() {
    assert_eq!(part2(&vec![1, -1]), 0);
    assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
    assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
    assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
}

fn main() -> io::Result<()> {
    let raw = read("input1.txt")?;
    let input = raw.split_whitespace().map(|s| s.parse::<i64>().map_err(|_| io::Error::from(io::ErrorKind::InvalidData))).collect::<io::Result<Vec<i64>>>()?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}
