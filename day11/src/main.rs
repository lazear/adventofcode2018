use std::io;
use util;

fn power(x: usize, y: usize, serial: usize) -> i32 {
    let rack = x + 10;
    let mut pl = rack * y + serial;
    pl *= rack;
    pl = (pl - (pl % 100)) / 100 % 10;
    pl as i32 - 5
}

fn part1(serial: usize) -> (usize, usize) {
    let mut max = (0, 0, 0);
    for x in 0..297 {
        for y in 0..297 {
            let mut sum = 0;
            for i in x..x + 3 {
                for j in y..y + 3 {
                    sum += power(i + 1, j + 1, serial);
                }
            }
            if sum > max.0 {
                max = (sum, x + 1, y + 1);
            }
        }
    }
    (max.1, max.2)
}

fn part2(serial: usize) -> (usize, usize, usize) {
    let mut max = (0, 0, 0, 0);
    for dim in 1..=300 {
        for x in 0..300 - dim {
            for y in 0..300 - dim {
                let mut sum = 0;
                for i in x..x + dim {
                    for j in y..y + dim {
                        sum += power(i + 1, j + 1, serial);
                    }
                }
                if sum > max.0 {
                    max = (sum, x + 1, y + 1, dim);
                }
            }
        }
    }

    (max.1, max.2, max.3)
}

#[test]
fn part1_test() {
    assert_eq!(power(3, 5, 8), 4);
    assert_eq!(power(122, 79, 57), -5);
    assert_eq!(power(217, 196, 39), 0);
    assert_eq!(power(101, 153, 71), 4);
    assert_eq!(part1(18), (33, 45));
    assert_eq!(part1(42), (21, 61));
}

#[test]
fn part2_test() {
    assert_eq!(part2(18), (90, 269, 16));
    assert_eq!(part2(42), (232, 251, 12));
}

fn main() -> io::Result<()> {
    println!("Part 1: {:?}", part1(6548));
    println!("Part 2: {:?}", part2(6548));
    Ok(())
}
