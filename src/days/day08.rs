use std::fs;
use std::collections::HashSet;

#[derive(Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

#[derive(PartialEq)]
enum Status {
    End,
    Loop
}

impl From<&str> for Instr {
    fn from(code: &str) -> Self {
        let mut tokens = code.split_whitespace();
        let inst = tokens.next().unwrap();
        let value = tokens.next().unwrap().parse::<i32>().unwrap();
        match inst {
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            "nop" => Self::Nop(value),
            _ => unreachable!()
        }
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day08.txt")
        .expect("Something went wrong reading the file");

    let acc = impl_first_star(&contents);

    println!("day  8.1 - value of accumulator at start of loop: {}", acc);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day08.txt")
        .expect("Something went wrong reading the file");

    let acc = impl_second_star(&contents).unwrap();

    println!("day  8.2 - value of accumulator after fix: {}", acc);
}

fn impl_first_star(contents: &str) -> i32 {
    let instructions = parse_code(contents);
    compute(&instructions).0
}

fn impl_second_star(contents: &str) -> Option<i32> {
    let instructions = parse_code(contents);
    for i in 0..instructions.len() {
        let mut fixed_instr = instructions.clone();
        match instructions[i] {
            Instr::Acc(_) => continue,
            Instr::Jmp(value) => fixed_instr[i] = Instr::Nop(value),
            Instr::Nop(value) => fixed_instr[i] = Instr::Jmp(value)
        }
        let (acc, status) = compute(&fixed_instr);
        if status == Status::End {
            return Some(acc)
        }
    }
    None
}

fn parse_code(contents: &str) -> Vec<Instr> {
    contents
        .lines()
        .map(Instr::from)
        .collect()
}

fn compute(instructions: &[Instr]) -> (i32, Status) {
    let mut acc: i32 = 0;
    let mut pos: i32 = 0;
    let last = instructions.len() as i32;
    let mut executed = HashSet::new();
    while pos >= 0 && pos < last && executed.insert(pos) {
        match instructions[pos as usize] {
            Instr::Acc(value) => { acc += value; pos += 1; },
            Instr::Jmp(value) => pos += value,
            Instr::Nop(_) => pos += 1
        }
    }
    (acc, if pos < last { Status::Loop } else { Status::End })
}

#[test]
fn test0_first_star() {
    let contents =
    "nop +0\n\
     acc +1\n\
     jmp +4\n\
     acc +3\n\
     jmp -3\n\
     acc -99\n\
     acc +1\n\
     jmp -4\n\
     acc +6";
    assert_eq!(impl_first_star(contents), 5);
}

#[test]
fn test0_second_star() {
    let contents =
    "nop +0\n\
     acc +1\n\
     jmp +4\n\
     acc +3\n\
     jmp -3\n\
     acc -99\n\
     acc +1\n\
     jmp -4\n\
     acc +6";
    assert_eq!(impl_second_star(contents), Some(8));
}