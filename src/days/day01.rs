use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day01.txt")
        .expect("Something went wrong reading the file");

    let product = impl_first_star(&contents).unwrap();

    println!("day  1.1 - product of the two entries that sum to 2020: {}", product);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day01.txt")
        .expect("Something went wrong reading the file");

    let product = impl_second_star(&contents).unwrap();

    println!("day  1.2 - product of the three entries that sum to 2020: {}", product);
}

fn impl_first_star(contents: &str) -> Option<u32> {
    let entries = parse_entries(contents);

    for (i, e) in entries.iter().enumerate() {
        for f in entries.iter().skip(i + 1) {
            if e + f == 2020 { return Some(e * f) }
        }
    }

    None
}

fn impl_second_star(contents: &str) -> Option<u32> {
    let entries = parse_entries(contents);

    for (i, e) in entries.iter().enumerate() {
        for (j, f) in entries.iter().enumerate().skip(i + 1) {
            for g in entries.iter().skip(j + 1) {
                if e + f + g == 2020 { return Some(e * f * g) }
            }
        }
    }

    None
}

fn parse_entries(contents: &str) -> Vec<u32> {
    contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

#[test]
fn test0_first_star() {
    let entries =
        "1721\n\
         979\n\
         366\n\
         299\n\
         675\n\
         1456";
    assert_eq!(impl_first_star(entries), Some(514579));
}

#[test]
fn test0_second_star() {
    let entries =
        "1721\n\
         979\n\
         366\n\
         299\n\
         675\n\
         1456";
    assert_eq!(impl_second_star(entries), Some(241861950));
}
