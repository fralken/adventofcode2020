use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day05.txt")
        .expect("Something went wrong reading the file");

    let highest_seat_id = impl_first_star(&contents);

    println!("day  5.1 - highest seat ID: {}", highest_seat_id);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day05.txt")
        .expect("Something went wrong reading the file");

    let seat_id = impl_second_star(&contents);

    println!("day  5.2 - my seat ID: {}", seat_id);
}

fn impl_first_star(contents: &str) -> usize {
    *parse_boarding_passes(contents).iter().max().unwrap()
}

fn impl_second_star(contents: &str) -> usize {
    let boarding_passes = parse_boarding_passes(contents);
    let mut pass = 0;
    for i in 0..1022 {
        if boarding_passes.contains(&i) &&
            !boarding_passes.contains(&(i + 1)) &&
            boarding_passes.contains(&(i + 2)) {
            pass = i + 1;
            break;
        }
    }
    pass
}

fn parse_boarding_passes(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|s|
            s.chars().fold(0 as usize, |a, c|
                if c == 'B' || c == 'R' { a * 2 + 1 }
                else if c == 'F' || c == 'L' { a * 2 }
                else { a }
            )
        )
        .collect()
}

#[test]
fn test0_first_star() {
    let contents =
        "BFFFBBFRRR\n\
         FFFBBBFRRR\n\
         BBFFBBFRLL";
    assert_eq!(parse_boarding_passes(contents), vec![567, 119, 820]);
}