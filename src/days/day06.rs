use std::fs;
use std::collections::HashSet;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day06.txt")
        .expect("Something went wrong reading the file");

    let counts = impl_first_star(&contents);

    println!("day  6.1 - counts of 'yes' questions (anyone): {}", counts);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day06.txt")
        .expect("Something went wrong reading the file");

    let counts = impl_second_star(&contents);

    println!("day  6.2 - counts of 'yes' questions (everyone): {}", counts);
}

fn impl_first_star(contents: &str) -> usize {
    count_questions(contents, |a, b| a | b)
}

fn impl_second_star(contents: &str) -> usize {
    count_questions(contents, |a, b| a & b)
}

fn count_questions(contents: &str,
                   op: impl Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>) -> usize {
    contents
        .split("\n\n")
        .map(|group|
            group
                .lines()
                .map(|line|
                    line
                        .chars()
                        .collect::<HashSet<char>>()
                )
                .fold(None, |acc, set|
                    match acc {
                        None => Some(set),
                        Some(a) => Some(op(&a, &set))
                    }
                )
                .unwrap()
                .len()
        ).sum()
}

#[test]
fn test0_first_star() {
    let contents =
        "abc\n\
         \n\
         a\n\
         b\n\
         c\n\
         \n\
         ab\n\
         ac\n\
         \n\
         a\n\
         a\n\
         a\n\
         a\n\
         \n\
         b";
    assert_eq!(impl_first_star(contents), 11);
}

#[test]
fn test0_second_star() {
    let contents =
        "abc\n\
         \n\
         a\n\
         b\n\
         c\n\
         \n\
         ab\n\
         ac\n\
         \n\
         a\n\
         a\n\
         a\n\
         a\n\
         \n\
         b";
    assert_eq!(impl_second_star(contents), 6);
}