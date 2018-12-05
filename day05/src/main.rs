extern crate util;
use std::io;

fn part1(data: String) -> Result<usize, ()> {
    let mut s = data;
    let mut j = 0;
    'outer: loop {
        for i in j..s.len() - 2 {
            let a = &s[i..i + 1];
            let b = &s[i + 1..i + 2];
            if a != b && a.eq_ignore_ascii_case(b) {
                let mut x = String::from(&s[0..i]);
                x.push_str(&s[i + 2..]);
                s = x;
                j = i - 1;
                continue 'outer;
            }
        }
        break;
    }
    Ok(s.len())
}

fn part2(data: &str) -> Result<usize, ()> {
    let mut best = ('a', data.len());
    for i in ('a' as u32)..=('z' as u32) {
        let c = std::char::from_u32(i).ok_or(())?;
        let s = data
            .chars()
            .filter(|a| !a.eq_ignore_ascii_case(&c))
            .collect::<String>();
        let r = part1(s)?;
        if r < best.1 {
            best = (c, r);
        }
    }
    println!("{:?}", best);
    Ok(best.1)
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
