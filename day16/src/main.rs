use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs;
use std::io::prelude::*;
use std::mem::{discriminant, Discriminant};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Registers([i64; 4]);
impl Registers {
    pub fn from_slice(slice: &[i64]) -> Self {
        Registers([slice[0], slice[1], slice[2], slice[3]])
    }
}

impl std::ops::Index<Register> for Registers {
    type Output = i64;
    fn index(&self, register: Register) -> &Self::Output {
        let idx: usize = register.into();
        &self.0[idx]
    }
}

impl std::ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        let idx: usize = register.into();
        &mut self.0[idx]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl TryFrom<i64> for Register {
    type Error = i64;
    fn try_from(val: i64) -> Result<Register, Self::Error> {
        match val {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            _ => Err(val),
        }
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
        }
    }
}

type Immediate = i64;

// In format of
// (Register A, Register B, Register C)
// (Register A, Value B, Register C), etc
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Addr(Register, Register, Register),
    Addi(Register, Immediate, Register),
    Mulr(Register, Register, Register),
    Muli(Register, Immediate, Register),
    Banr(Register, Register, Register),
    Bani(Register, Immediate, Register),
    Borr(Register, Register, Register),
    Bori(Register, Immediate, Register),
    Setr(Register, Immediate, Register),
    Seti(Immediate, Immediate, Register),
    Gtir(Immediate, Register, Register),
    Gtri(Register, Immediate, Register),
    Gtrr(Register, Register, Register),
    Eqir(Immediate, Register, Register),
    Eqri(Register, Immediate, Register),
    Eqrr(Register, Register, Register),
}

impl Opcode {
    pub fn execute(self, mut reg: Registers) -> Registers {
        use Opcode::*;
        match self {
            Addr(a, b, c) => reg[c] = reg[a] + reg[b],
            Addi(a, b, c) => reg[c] = reg[a] + b,
            Mulr(a, b, c) => reg[c] = reg[a] * reg[b],
            Muli(a, b, c) => reg[c] = reg[a] * b,
            Banr(a, b, c) => reg[c] = reg[a] & reg[b],
            Bani(a, b, c) => reg[c] = reg[a] & b,
            Borr(a, b, c) => reg[c] = reg[a] | reg[b],
            Bori(a, b, c) => reg[c] = reg[a] | b,
            Setr(a, _, c) => reg[c] = reg[a],
            Seti(a, _, c) => reg[c] = a,
            Gtir(a, b, c) => reg[c] = if a > reg[b] { 1 } else { 0 },
            Gtri(a, b, c) => reg[c] = if reg[a] > b { 1 } else { 0 },
            Gtrr(a, b, c) => reg[c] = if reg[a] > reg[b] { 1 } else { 0 },
            Eqir(a, b, c) => reg[c] = if a == reg[b] { 1 } else { 0 },
            Eqri(a, b, c) => reg[c] = if reg[a] == b { 1 } else { 0 },
            Eqrr(a, b, c) => reg[c] = if reg[a] == reg[b] { 1 } else { 0 },
        }
        reg
    }
}

impl Into<i64> for Opcode {
    fn into(self) -> i64 {
        use Opcode::*;
        match self {
            Addr(a, b, c) => 0,
            Addi(a, b, c) => 1,
            Mulr(a, b, c) => 2,
            Muli(a, b, c) => 3,
            Banr(a, b, c) => 4,
            Bani(a, b, c) => 5,
            Borr(a, b, c) => 6,
            Bori(a, b, c) => 7,
            Setr(a, _, c) => 8,
            Seti(a, _, c) => 9,
            Gtir(a, b, c) => 10,
            Gtri(a, b, c) => 11,
            Gtrr(a, b, c) => 12,
            Eqir(a, b, c) => 13,
            Eqri(a, b, c) => 14,
            Eqrr(a, b, c) => 15,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpcodeIter {
    a: i64,
    b: i64,
    c: i64,
    idx: i64,
}

impl Iterator for OpcodeIter {
    type Item = Opcode;
    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! reg {
            ($ex:expr) => {
                Register::try_from($ex).ok()?
            };
        }
        use Opcode::*;
        let n = self.idx;
        self.idx += 1;
        match n {
            0 => Some(Addr(reg!(self.a), reg!(self.b), reg!(self.c))),
            1 => Some(Addi(reg!(self.a), self.b, reg!(self.c))),
            2 => Some(Mulr(reg!(self.a), reg!(self.b), reg!(self.c))),
            3 => Some(Muli(reg!(self.a), self.b, reg!(self.c))),
            4 => Some(Banr(reg!(self.a), reg!(self.b), reg!(self.c))),
            5 => Some(Bani(reg!(self.a), self.b, reg!(self.c))),
            6 => Some(Borr(reg!(self.a), reg!(self.b), reg!(self.c))),
            7 => Some(Bori(reg!(self.a), self.b, reg!(self.c))),
            8 => Some(Setr(reg!(self.a), self.b, reg!(self.c))),
            9 => Some(Seti(self.a, self.b, reg!(self.c))),
            10 => Some(Gtir(self.a, reg!(self.b), reg!(self.c))),
            11 => Some(Gtri(reg!(self.a), self.b, reg!(self.c))),
            12 => Some(Gtrr(reg!(self.a), reg!(self.b), reg!(self.c))),
            13 => Some(Eqir(self.a, reg!(self.b), reg!(self.c))),
            14 => Some(Eqri(reg!(self.a), self.b, reg!(self.c))),
            15 => Some(Eqrr(reg!(self.a), reg!(self.b), reg!(self.c))),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sample {
    before: Registers,
    after: Registers,
    instr: Instr,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Instr {
    code: i64,
    a: i64,
    b: i64,
    c: i64,
}
impl Sample {
    pub fn matches(&self) -> Vec<Opcode> {
        let mut it = OpcodeIter {
            a: self.instr.a,
            b: self.instr.b,
            c: self.instr.c,
            idx: 0,
        };
        (0..16)
            .map(|_| it.next())
            .filter_map(|x| x)
            .filter(|op| op.execute(self.before) == self.after)
            .collect()
    }
}

impl Instr {
    pub fn into_opcode(self, map: &HashMap<i64, i64>) -> Option<Opcode> {
        let mut it = OpcodeIter {
            a: self.a,
            b: self.b,
            c: self.c,
            idx: *map.get(&self.code)?,
        };
        it.next()
    }
}

pub fn part_1(samples: &[Sample]) -> usize {
    samples.iter().filter(|s| s.matches().len() >= 3).count()
}

pub fn part_2(samples: &[Sample], instrs: &[Instr]) -> i64 {
    let mut map: HashMap<i64, HashSet<i64>> = HashMap::new();
    for s in samples {
        map.entry(s.instr.code)
            .or_insert_with(HashSet::new)
            .extend(s.matches().into_iter().map(Into::<i64>::into));
    }

    let mut known: HashMap<i64, i64> = HashMap::new();
    for _ in 0..16 {
        // Find all instruction #s with i possible discriminants
        // Collect into a vector so that we're not violating borrowck rules
        let work = map
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, _)| k)
            .copied()
            .collect::<Vec<_>>();
        for instr in work {
            // We know that there is a single entry, so this should never panic.
            let discrim = map.remove(&instr).unwrap().drain().next().unwrap();
            known.insert(instr, discrim);
            for (_, vals) in map.iter_mut() {
                vals.remove(&discrim);
            }
        }
    }
    assert_eq!(map.len(), 0);
    assert_eq!(known.len(), 16);
    instrs
        .into_iter()
        .filter_map(|s| s.into_opcode(&known))
        .fold(Registers::default(), |regs, op| op.execute(regs))[Register::R0]
}

fn parse(path: &str) -> std::io::Result<(Vec<Sample>, Vec<Instr>)> {
    let input = util::read_lines(path)?;
    let mut v = Vec::new();
    let mut p = Vec::new();

    for chunk in input.chunks(4) {
        // dbg!(&chunk);
        if chunk[0].starts_with("Before") {
            let mut sample = Sample::default();

            let (start, end) = (chunk[0].find('[').unwrap(), chunk[0].find(']').unwrap());
            let r = (&chunk[0])[start + 1..end]
                .split(',')
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            sample.before = Registers::from_slice(&r);
            let (start, end) = (chunk[2].find('[').unwrap(), chunk[0].find(']').unwrap());
            let r = (&chunk[2])[start + 1..end]
                .split(',')
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            sample.after = Registers::from_slice(&r);

            let r = chunk[1]
                .split(' ')
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            sample.instr.code = r[0];
            sample.instr.a = r[1];
            sample.instr.b = r[2];
            sample.instr.c = r[3];

            v.push(sample);
        } else {
            for i in 0..chunk.len() {
                if chunk[i].starts_with(char::is_numeric) {
                    let r = chunk[i]
                        .split(' ')
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    let instr = Instr {
                        code: r[0],
                        a: r[1],
                        b: r[2],
                        c: r[3],
                    };
                    p.push(instr);
                }
            }
        }
    }
    Ok((v, p))
}

fn main() {
    println!("Hello, world!");

    let (samples, program) = parse("input.txt").unwrap();
    println!("Part 1: {}", part_1(&samples));
    println!("Part 2: {}", part_2(&samples, &program));
}
