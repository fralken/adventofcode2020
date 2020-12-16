use std::fs;
use std::collections::HashMap;
use text_io::scan;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day14.txt")
        .expect("Something went wrong reading the file");

    let sum = impl_first_star(&contents);

    println!("day 14.1 - sum of memory values (version 1): {}", sum);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day14.txt")
        .expect("Something went wrong reading the file");

    let sum = impl_second_star(&contents);

    println!("day 14.2 - sum of memory values (version 2): {}", sum);
}

fn impl_first_star(contents: &str) -> u64 {
    execute(contents, mask_value)
}

fn impl_second_star(contents: &str) -> u64 {
    execute(contents, mask_address)
}

fn execute<F>(contents: &str, masker: F) -> u64
    where F: Fn(&mut HashMap<u64, u64>, &str, u64, u64) {
    let (_, memory) = contents
        .lines()
        .fold((String::new(), HashMap::new()), |(mut mask, mut mem), line| {
            if line.starts_with("mask") {
                scan!(line.bytes() => "mask = {}", mask);
            } else {
                let address: u64;
                let value: u64;
                scan!(line.bytes() => "mem[{}] = {}", address, value);
                masker(&mut mem, &mask, address, value);
            }
            (mask, mem)
        });

    memory.values().sum()
}

fn mask_value(mem: &mut HashMap<u64, u64>, mask: &str, address: u64, mut value: u64) {
    let mut bit = 1;
    for v in mask.chars().rev() {
        match v {
            'X' => (),
            '1' => value |= bit,
            '0' => value &= !bit,
            _ => panic!("invalid bit mask")
        };
        bit <<= 1;
    };
    mem.insert(address, value);
}

fn mask_address(mem: &mut HashMap<u64, u64>, mask: &str, mut address: u64, value: u64) {
    let mut bit = 1;
    let mut floating = Vec::new();
    for v in mask.chars().rev() {
        match v {
            'X' => { address &= !bit; floating.push(bit) },
            '1' => address |= bit,
            '0' => (),
            _ => panic!("invalid bit mask")
        };
        bit <<= 1;
    };

    for m in 0..1<<floating.len() {
        let mut new_address = address;
        for (i, f) in floating.iter().enumerate() {
            match m >> i & 1 {
                0 => new_address &= !f,
                1 => new_address |= f,
                _ => unreachable!()
            }
        }

        mem.insert(new_address, value);
    };
}

#[test]
fn test0_first_star() {
    let contents =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
         mem[8] = 11\n\
         mem[7] = 101\n\
         mem[8] = 0";
    assert_eq!(impl_first_star(contents), 165);
}

#[test]
fn test0_second_star() {
    let contents =
        "mask = 000000000000000000000000000000X1001X\n\
         mem[42] = 100\n\
         mask = 00000000000000000000000000000000X0XX\n\
         mem[26] = 1";
    assert_eq!(impl_second_star(contents), 208);
}
