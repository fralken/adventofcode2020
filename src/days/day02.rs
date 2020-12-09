use std::fs;
use text_io::scan;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day02.txt")
        .expect("Something went wrong reading the file");

    let valid = impl_first_star(&contents);

    println!("day  2.1 - valid passwords for first policy: {}", valid);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day02.txt")
        .expect("Something went wrong reading the file");

    let valid = impl_second_star(&contents);

    println!("day  2.2 - valid passwords for second policy: {}", valid);
}

fn impl_first_star(contents: &str) -> usize {
    contents
        .lines()
        .map(parse)
        .filter(|p| {
            let count = p.3.chars().filter(|c| c == &p.2).count();
            count >= p.0 && count <= p.1
        })
        .count()
}

fn impl_second_star(contents: &str) -> usize {
    contents
        .lines()
        .map(parse)
        .filter(|p|
            (p.3.chars().nth(p.0 - 1) == Some(p.2)) !=
                (p.3.chars().nth(p.1 - 1) == Some(p.2))
        )
        .count()
}

fn parse(line: &str) -> (usize, usize, char, String) {
    let min: usize;
    let max: usize;
    let c: char;
    let pwd: String;
    scan!(line.bytes() => "{}-{} {}: {}", min, max, c, pwd);
    (min, max, c, pwd)
}

#[test]
fn test0_first_star() {
    let passwords =
        "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc";
    assert_eq!(impl_first_star(passwords), 2);
}

#[test]
fn test0_second_star() {
    let passwords =
        "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc";
    assert_eq!(impl_second_star(passwords), 1);
}
