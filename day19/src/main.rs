use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    Parsing,
    InvalidRegister(i64),
    IOError(std::io::Error),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorKind::IOError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Registers([i64; 6]);
impl Registers {
    pub fn from_slice(slice: &[i64]) -> Self {
        Registers([slice[0], slice[1], slice[2], slice[3], slice[4], slice[5]])
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
    R4,
    R5,
}

impl TryFrom<i64> for Register {
    type Error = ErrorKind;
    fn try_from(val: i64) -> Result<Register, Self::Error> {
        match val {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            _ => Err(ErrorKind::InvalidRegister(val)),
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
            Register::R4 => 4,
            Register::R5 => 5,
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
            Addr(_, _, _) => 0,
            Addi(_, _, _) => 1,
            Mulr(_, _, _) => 2,
            Muli(_, _, _) => 3,
            Banr(_, _, _) => 4,
            Bani(_, _, _) => 5,
            Borr(_, _, _) => 6,
            Bori(_, _, _) => 7,
            Setr(_, _, _) => 8,
            Seti(_, _, _) => 9,
            Gtir(_, _, _) => 10,
            Gtri(_, _, _) => 11,
            Gtrr(_, _, _) => 12,
            Eqir(_, _, _) => 13,
            Eqri(_, _, _) => 14,
            Eqrr(_, _, _) => 15,
        }
    }
}

impl std::str::FromStr for Opcode {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tk = s.split_whitespace().collect::<Vec<&str>>();
        let op = tk.remove(0);
        let ints = tk
            .into_iter()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;
        let idx = match op {
            "addr" => 0,
            "addi" => 1,
            "mulr" => 2,
            "muli" => 3,
            "banr" => 4,
            "bani" => 5,
            "borr" => 6,
            "bori" => 7,
            "setr" => 8,
            "seti" => 9,
            "gtir" => 10,
            "gtri" => 11,
            "gtrr" => 12,
            "eqir" => 13,
            "eqri" => 14,
            "eqrr" => 15,
            _ => panic!(),
        };

        Ok(OpcodeIter {
            a: ints[0],
            b: ints[1],
            c: ints[2],
            idx,
        }
        .next()
        .expect("invalid args for opcode"))
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

#[derive(Copy, Clone, Debug)]
pub struct Machine {
    registers: Registers,
    ip: Register,
}

impl Machine {
    pub const fn new(ip: Register) -> Machine {
        Machine {
            registers: Registers([0, 0, 0, 0, 0, 0]),
            ip,
        }
    }

    pub fn run(&mut self, opcodes: &[Opcode], verbose: bool) {
        let mut ip: usize = 0;
        loop {
            if ip >= opcodes.len() {
                break;
            }
            // When the instruction pointer is bound to a register, its value
            // is written to that register just before each instruction is executed
            self.registers[self.ip] = ip as i64;
            let new = opcodes[ip].execute(self.registers);
            if verbose {
                print!("ip={} {:?} {:?}", ip, self.registers.0, opcodes[ip]);
            }
            // the value of that register is written back to the instruction pointer
            // immediately after each instruction finishes execution. Afterward,
            // move to the next instruction by adding one to the instruction pointer,
            // even if the value in the instruction pointer was just updated by an
            // instruction. (Because of this, instructions must effectively set the
            // instruction pointer to the instruction before the one they want executed
            //  next.)
            self.registers = new;
            ip = self.registers[self.ip] as usize;
            ip += 1;
            if verbose {
                println!("{:?}", self.registers);
            }
        }
    }
}

pub fn parse(path: &str) -> std::io::Result<(Register, Vec<Opcode>)> {
    let input = fs::read_to_string(path)?;
    let mut reg = Register::R0;
    let mut v = Vec::new();
    for line in input.lines() {
        if line.starts_with('#') {
            reg = Register::try_from(line.trim_start_matches("#ip ").parse::<i64>().unwrap())
                .unwrap();
        } else {
            v.push(line.parse::<Opcode>().unwrap());
        }
    }

    Ok((reg, v))
}

fn main() {
    println!("Hello, world!");

    let (reg, program) = parse("input.txt").unwrap();

    let mut machine = Machine::new(reg);
    machine.run(&program, false);
    dbg!(machine);

    let mut machine = Machine::new(reg);
    machine.registers[Register::R0] = 1;
    machine.run(&program, false);
    dbg!(machine);
    // println!("Part 1: {}", part_1(&samples));
}
