use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use util;

///Directed acyclic graph
#[derive(Default, Debug, Clone, PartialEq)]
struct Node {
    id: char,
    incoming: Vec<char>,
    outgoing: Vec<char>,
}

#[derive(Default, Debug, Copy, Clone)]
struct Job {
    letter: Option<char>,
    clock: u32,
}

impl Node {
    fn new(id: char) -> Node {
        Node {
            id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}

fn parse_graph(data: &[String]) -> Option<HashMap<char, Node>> {
    let mut v = Vec::new();
    for line in data {
        let line = line
            .split(' ')
            .map(|s| s.chars().take(1).next())
            .collect::<Option<Vec<char>>>()?;
        v.push((line[1], line[7]));
    }
    let mut graph: HashMap<char, Node> = HashMap::new();

    for (i, o) in v {
        graph.entry(i).or_insert(Node::new(i)).outgoing.push(o);
        graph.entry(o).or_insert(Node::new(o)).incoming.push(i);
    }
    Some(graph)
}

fn part1(data: &[String]) -> Option<String> {
    let graph = parse_graph(data)?;
    let mut steps: Vec<char> = Vec::new();

    // Hashset of ready steps
    let mut ready: HashSet<char> = graph
        .iter()
        .filter_map(|(id, node)| {
            if node.incoming.len() == 0 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    'outer: loop {
        if ready.len() == 0 {
            break;
        }
        // Alphabetically sort our list of ready steps
        let mut available = ready.iter().cloned().collect::<Vec<char>>();
        available.sort();

        for a in &available {
            let node = graph.get(a)?;
            for child_id in &node.outgoing {
                if steps.contains(child_id) {
                    // we've already done this child
                    continue;
                }
                if graph
                    .get(child_id)?
                    .incoming
                    .iter()
                    .filter(|id| !(steps.contains(id) || *id == a))
                    .count()
                    == 0
                {
                    ready.insert(*child_id);
                }
            }
            ready.remove(a);
            steps.push(*a);
            // Continue to the outer loop, because this child node becoming ready
            // may allow other nodes that have alphabetic priority to become ready
            continue 'outer;
        }
    }
    Some(steps.iter().cloned().collect::<String>())
}

fn duration(letter: char, additional: u32) -> u32 {
    (letter.to_ascii_uppercase() as u32 - 'A' as u32) + additional
}

fn part2(data: &[String], workers: usize, additional: u32) -> Option<u32> {
    let graph = parse_graph(data)?;

    let mut ready: HashSet<char> = graph
        .iter()
        .filter_map(|(id, node)| {
            if node.incoming.len() == 0 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    let mut remaining = graph.keys().cloned().collect::<HashSet<char>>();
    let mut ticks = 0;
    let mut done: Vec<char> = Vec::new();
    let mut jobs = (0..workers + 1)
        .map(|_| Job::default())
        .collect::<Vec<Job>>();

    // Outer loop represents 1 second of work
    let mut queue: VecDeque<char> = VecDeque::new();
    loop {
        // Loop through all our jobs twice, first to see if any have finished - which may open up
        // new opportunities. There's an edge case on my input here, where the last worker
        // in the queue frees up 3 possible tasks, but the next iteration skips them.
        for job in jobs.iter_mut() {
            if job.clock > 0 {
                job.clock -= 1;
            } else {
                // Did we just finish a job?
                match job.letter.take() {
                    // We had a job to work on, just finished
                    Some(c) => {
                        let node = graph.get(&c)?;
                        // Are any of our direct outgoing nodes now available to work on?
                        for child_id in &node.outgoing {
                            if done.contains(child_id) {
                                // we've already done this child
                                continue;
                            }
                            if graph
                                .get(child_id)?
                                .incoming
                                .iter()
                                .filter(|id| !(done.contains(id) || *id == &c))
                                .count()
                                == 0
                            {
                                ready.insert(*child_id);
                            }
                        }
                        done.push(c);
                        remaining.remove(&c);
                        job.letter = None;
                    }
                    // No previous job, we've been idle
                    None => (),
                }
            }
        }

        for job in jobs.iter_mut() {
            // There are ready jobs in the queue.
            if !ready.is_empty() {
                let mut available = ready.iter().cloned().collect::<Vec<char>>();
                available.sort_by(|&a, &b| duration(a, additional).cmp(&duration(b, additional)));
                queue.extend(VecDeque::from(available));
                ready.clear();
            }

            // Take a job if we don't have one.
            if job.letter.is_none() {
                job.letter = queue.pop_front();
                match job.letter {
                    // Start a new job
                    Some(c) => {
                        // println!("{}:#{} taking {}", ticks,i,c);
                        job.clock = duration(c, additional);
                        ready.remove(&c);
                    }
                    // Stay idle
                    None => (),
                }
            }
        }

        println!(
            "{:04} {:?} {}",
            ticks,
            jobs.iter()
                .map(|j| j.letter.unwrap_or('.'))
                .collect::<Vec<_>>(),
            done.iter().cloned().collect::<String>()
        );

        if remaining.is_empty() {
            break;
        }
        ticks += 1;
    }

    Some(ticks)
}

#[test]
fn part1_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part1(&data), Some(String::from("CABDFE")));
}

#[test]
fn part2_test() {
    let data = util::read_lines("test1.txt").unwrap();
    assert_eq!(part2(&data, 1, 0), Some(15));
}

fn main() -> io::Result<()> {
    let data = util::read_lines("input.txt")?;
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data, 5, 60));
    Ok(())
}
