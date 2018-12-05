use std::io;
extern crate util;

fn part1(data: &[String]) -> i64 {
    let mut twos = 0;
    let mut threes = 0;
    for word in data {
        let mut array = [0u8; 26];
        word.chars()
            .for_each(|c| array[(c as usize - 'a' as usize)] += 1);
        twos += array
            .iter()
            .fold(0, |acc, &x| acc + if x == 2 { 1 } else { 0 })
            .min(1);
        threes += array
            .iter()
            .fold(0, |acc, &x| acc + if x == 3 { 1 } else { 0 })
            .min(1);
    }
    twos * threes
}

fn part2(data: &[String]) -> Option<String> {
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let x = data[i]
                .chars()
                .zip(data[j].chars())
                .fold(0u8, |acc, (a, b)| acc + if a == b { 0 } else { 1 });
            if x == 1 {
                return Some(
                    data[i]
                        .chars()
                        .zip(data[j].chars())
                        .filter(|(a, b)| a == b)
                        .fold(String::new(), |mut acc, (c, _)| {
                            acc.push(c);
                            acc
                        }),
                );
            }
        }
    }
    None
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), 12);
}

#[test]
fn part2_test() {
    let data = util::read_lines("test2.txt").unwrap();
    assert_eq!(part2(&data), Some(String::from("fgij")));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("1: {:?}", part1(&data));
    println!("2: {:?}", part2(&data));
    Ok(())
}
