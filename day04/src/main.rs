extern crate util;
use self::ParseError::*;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Action {
    Wake,
    Sleep,
    Shift(Guard),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Event {
    date: u16,
    time: i16,
    action: Action,
    guard: Option<Guard>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Guard(u16);

#[derive(Debug, Clone)]
struct Log {
    guards: HashMap<u16, Guard>,
    events: HashMap<Guard, Vec<Event>>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum ParseError {
    InvalidDate,
    InvalidTime,
    InvalidGuard,
    InvalidAction,
}

fn date_wrap(date: u16) -> u16 {
    // 1518 is a leap year
    let days = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = date / 100;
    let mut day = date % 100;
    if day > days[month as usize - 1] {
        ((month + 1) % 12) * 100 + (day - days[month as usize - 1])
    } else {
        date
    }
}

#[test]
fn date_wrap_test() {
    assert_eq!(date_wrap(1032), 1101);
    assert_eq!(date_wrap(0229), 0229);
    assert_eq!(date_wrap(1232), 0101);
}

fn parse_log(data: &Vec<String>) -> Result<HashMap<Guard, [u16; 60]>, ParseError> {
    let mut map: HashMap<u16, Vec<Event>> = HashMap::new();
    let mut guards: HashMap<Guard, HashMap<u16, Vec<(i16, Action)>>> = HashMap::new();

    for line in data {
        let line = line
            .trim_matches('[')
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mut date = line[0]
            .split('-')
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u16>()
            .map_err(|_| InvalidDate)?;
        let mut time = line[1]
            .trim_matches(']')
            .split(':')
            .collect::<Vec<&str>>()
            .join("")
            .parse::<i16>()
            .map_err(|_| InvalidDate)?;

        if time >= 2300 {
            // Should only occur for guard shifts beginning before midnight,
            // we need to adjust the date the guard shift days (D-1) to the
            // date on which sleeping/waking events happen
            // Of course, we need to account for days > # of days in month
            date = date_wrap(date + 1);
            time -= 2360;
        }
        let action = match line[2] {
            "wakes" => Action::Wake,
            "falls" => Action::Sleep,
            "Guard" => Action::Shift(Guard(
                line[3]
                    .trim_matches('#')
                    .parse::<u16>()
                    .map_err(|_| InvalidGuard)?,
            )),
            _ => panic!("Unrecognized log entry!"),
        };

        let event = Event {
            date,
            time,
            action,
            guard: None,
        };
        map.entry(date).or_insert_with(Vec::new).push(event);
    }

    // Check to make sure we only have 1 guard per day
    for (k, mut v) in map.iter_mut() {
        let guard_id = v
            .iter()
            .filter_map(|ev| {
                if let Action::Shift(Guard(t)) = ev.action {
                    Some(t)
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>();
        assert_eq!(guard_id.len(), 1);
        let g = Guard(guard_id[0]);
        v.iter().for_each(|ev| {
            guards
                .entry(g)
                .or_insert_with(HashMap::new)
                .entry(ev.date)
                .or_insert_with(Vec::new)
                .push((ev.time, ev.action));
        });
    }

    let mut minutes: HashMap<Guard, [u16; 60]> = HashMap::new();
    for (&g, v) in guards.iter_mut() {
        for (_, ev) in v.iter_mut() {
            ev.sort_by(|a, b| a.0.cmp(&b.0));
            let mut mm = [0u16; 60];
            for (time, stat) in ev {
                match stat {
                    Action::Sleep => {
                        for t in *time..60 {
                            mm[t as usize] += 1;
                        }
                    }
                    Action::Wake => {
                        for t in *time..60 {
                            mm[t as usize] = 0;
                        }
                    }
                    _ => (),
                }
            }

            let clock = minutes.entry(g).or_insert([0u16; 60]);
            for i in 0..60 {
                clock[i] += mm[i];
            }
        }
    }
    Ok(minutes)
}

fn part1(data: &Vec<String>) -> Result<usize, ParseError> {
    let minutes = parse_log(data)?;

    let mut highest = (0u16, Guard(0), 0usize);

    for (&g, clock) in &minutes {
        let xs = clock.iter().map(|&u| u).collect::<Vec<u16>>();
        let total: u16 = xs.iter().map(|&u| u).sum();
        let max = xs.iter().map(|&u| u).max().unwrap();
        let mut minute = 0;
        for (i, &x) in xs.iter().enumerate() {
            if x == max {
                minute = i
            }
        }
        if total > highest.0 {
            highest = (total, g, minute);
        }
    }

    Ok(highest.2 * (highest.1).0 as usize)
}

fn part2(data: &Vec<String>) -> Result<usize, ParseError> {
    let minutes = parse_log(data)?;

    let mut highest = (0u16, Guard(0), 0usize);

    for (&g, clock) in &minutes {
        let xs = clock.iter().map(|&u| u).collect::<Vec<u16>>();
        let total: u16 = xs.iter().map(|&u| u).sum();
        let max = xs.iter().map(|&u| u).max().unwrap();
        let mut minute = 0;
        for (i, &x) in xs.iter().enumerate() {
            if x == max {
                minute = i
            }
        }
        if max > highest.0 {
            highest = (max, g, minute);
        }
    }

    Ok(highest.2 * (highest.1).0 as usize)
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), Ok(240));
}

#[test]
fn part2_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part2(&data), Ok(4455));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
    Ok(())
}
