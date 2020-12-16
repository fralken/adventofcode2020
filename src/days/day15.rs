use std::fs;
use std::collections::HashMap;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day15.txt")
        .expect("Something went wrong reading the file");

    let num = impl_first_star(&contents);

    println!("day 15.1 - 2020th number spoken: {}", num);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day15.txt")
        .expect("Something went wrong reading the file");

    let num = impl_second_star(&contents);

    println!("day 15.2 - 30000000th number spoken: {}", num);
}

fn impl_first_star(contents: &str) -> usize {
    compute(contents, 2020)
}

fn impl_second_star(contents: &str) -> usize {
    compute(contents, 30_000_000)
}

fn compute(contents: &str, limit: usize) -> usize {
    let numbers = parse_numbers(contents);
    let mut spoken = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i + 1))
        .collect::<HashMap<_,_>>();

    let mut last = *numbers.last().unwrap();
    for turn in numbers.len()..limit {
        let next = if let Some(last_turn) = spoken.get(&last) {
            turn - last_turn
        } else {
            0
        };
        spoken.insert(last, turn);
        last = next;
    }

    last
}

fn parse_numbers(contents: &str) -> Vec<usize> {
    contents
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[test]
fn test0_first_star() {
    let contents = "0,3,6";
    assert_eq!(impl_first_star(contents), 436);
}

#[test]
fn test1_first_star() {
    let contents = "1,3,2";
    assert_eq!(impl_first_star(contents), 1);
}

#[test]
fn test2_first_star() {
    let contents = "2,1,3";
    assert_eq!(impl_first_star(contents), 10);
}

#[test]
fn test3_first_star() {
    let contents = "1,2,3";
    assert_eq!(impl_first_star(contents), 27);
}

#[test]
fn test4_first_star() {
    let contents = "2,3,1";
    assert_eq!(impl_first_star(contents), 78);
}

#[test]
fn test5_first_star() {
    let contents = "3,2,1";
    assert_eq!(impl_first_star(contents), 438);
}

#[test]
fn test6_first_star() {
    let contents = "3,1,2";
    assert_eq!(impl_first_star(contents), 1836);
}

#[test]
fn test0_second_star() {
    let contents = "0,3,6";
    assert_eq!(impl_second_star(contents), 175594);
}

#[test]
fn test1_second_star() {
    let contents = "1,3,2";
    assert_eq!(impl_second_star(contents), 2578);
}

#[test]
fn test2_second_star() {
    let contents = "2,1,3";
    assert_eq!(impl_second_star(contents), 3544142);
}

#[test]
fn test3_second_star() {
    let contents = "1,2,3";
    assert_eq!(impl_second_star(contents), 261214);
}

#[test]
fn test4_second_star() {
    let contents = "2,3,1";
    assert_eq!(impl_second_star(contents), 6895259);
}

#[test]
fn test5_second_star() {
    let contents = "3,2,1";
    assert_eq!(impl_second_star(contents), 18);
}

#[test]
fn test6_second_star() {
    let contents = "3,1,2";
    assert_eq!(impl_second_star(contents), 362);
}
