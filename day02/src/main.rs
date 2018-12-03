use std::io;
extern crate util;

fn part1(data: &Vec<String>) -> i64 {
    let mut twos = 0;
    let mut threes = 0;
    for word in data {
        let mut array = [0u8; 26];
        let mut sum = 0;
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

fn part2(data: &Vec<String>) -> String {
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let x = data[i]
                .chars()
                .zip(data[j].chars())
                .fold(0u8, |acc, (a, b)| acc + if a == b { 0 } else { 1 });
            if x == 1 {
                return data[i]
                    .chars()
                    .zip(data[j].chars())
                    .filter(|(a, b)| a == b)
                    .fold(String::new(), |mut acc, (c, _)| {
                        acc.push(c);
                        acc
                    });
            }
        }
    }
    String::new()
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), 12);
}

#[test]
fn part2_test() {
    let data = util::read_lines("test2.txt").unwrap();
    assert_eq!(part2(&data), String::from("fgij"))
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let data = util::read_lines("input1.txt")?;
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
    Ok(())
}
