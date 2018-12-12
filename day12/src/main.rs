use std::collections::HashMap;
use std::io;
use util;

const CONVERGE: u32 = 10;

fn part1(data: &[String], generations: usize) -> Result<isize, ()> {
    let init = &data[0]
        .split(':')
        .map(str::trim)
        .skip(1)
        .collect::<Vec<&str>>();
    let mut states: HashMap<&str, char> = HashMap::new();

    data[2..].iter().for_each(|s| {
        states.insert(&s[0..5], if s.ends_with('#') { '#' } else { '.' });
    });

    let mut n = String::from("...");
    n.push_str(&init[0]);
    n.push_str("...");

    let mut last = 0;
    let mut diffs: HashMap<isize, u32> = HashMap::new();

    for gen in 1..=generations {
        let mut s = String::from("...");
        for i in 2..n.len() - 2 {
            let slice = &n[i - 2..=i + 2];
            match states.get(slice) {
                Some('#') => {
                    s.push('#');
                }
                _ => s.push('.'),
            }
        }
        s.push_str("...");
        n = s;

        // Our string grows by one '.' at both the beginning and end each generation
        let score = n
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum::<isize>();
        let e = diffs.entry(score as isize - last as isize).or_insert(0);
        if *e > CONVERGE {
            return Ok((generations - gen) as isize * (score - last) + score);
        } else {
            *e += 1;
        }
        last = score;
    }
    Ok(last)
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data, 20), Ok(325));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("Part 1: {:?}", part1(&data, 20));
    println!("Part 2: {:?}", part1(&data, 50_000_000_000));
    Ok(())
}
