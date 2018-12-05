extern crate util;
use std::io;

fn part1(data: String) -> Result<usize, ()> {
    let mut v = Vec::new();
    for c in data.chars() {
        match v.last() {
            None => v.push(c),
            Some(l) => {
                if c != *l && c.eq_ignore_ascii_case(l) {
                    v.pop();
                } else {
                    v.push(c);
                }
            }
        }
    }
    Ok(v.len())
}

fn part2(data: &str) -> Result<usize, ()> {
    let mut best = data.len();
    for i in ('a' as u32)..=('z' as u32) {
        let c = std::char::from_u32(i).ok_or(())?;
        let s = data
            .chars()
            .filter(|a| !a.eq_ignore_ascii_case(&c))
            .collect::<String>();
        best = best.min(part1(s)?);
    }
    Ok(best)
}

#[test]
fn part1_test() {
    let data = util::read("test1.txt").unwrap();
    assert_eq!(part1(data), Ok(10));
}

#[test]
fn part2_test() {
    let data = util::read("test1.txt").unwrap();
    assert_eq!(part2(&data), Ok(4));
}

fn main() -> io::Result<()> {
    let data = util::read("input.txt")?;
    println!("Part 1: {:?}", part1(data.clone()));
    println!("Part 2: {:?}", part2(&data));
    Ok(())
}
