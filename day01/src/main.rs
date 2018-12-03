extern crate util;
use std::collections::HashSet;
use std::io;

fn part1(data: &Vec<i64>) -> i64 {
    data.iter().fold(0, |acc, &x| acc + x)
}

fn part2(data: &Vec<i64>) -> i64 {
    let mut set = HashSet::<i64>::new();
    let mut r = 0;
    loop {
        for &x in data {
            if set.contains(&r) {
                return r;
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
    let input = util::read_lines("input.txt")?
        .iter()
        .map(|s| {
            s.parse::<i64>()
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))
        }).collect::<io::Result<Vec<i64>>>()?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}
