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

    let seat_id = impl_second_star(&contents).unwrap();

    println!("day  5.2 - my seat ID: {}", seat_id);
}

fn impl_first_star(contents: &str) -> usize {
    *parse_boarding_passes(contents).iter().max().unwrap()
}

fn impl_second_star(contents: &str) -> Option<usize> {
    let mut boarding_passes = parse_boarding_passes(contents);
    boarding_passes.sort_unstable();
    for i in 0..boarding_passes.len()-1 {
        if boarding_passes[i+1] - boarding_passes[i] == 2 {
            return Some(boarding_passes[i] + 1)
        }
    }
    None
}

fn parse_boarding_passes(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|s|
            s.chars().fold(0 as usize, |a, c|
                match c {
                    'B' | 'R' => a * 2 + 1,
                    'F' | 'L' => a * 2,
                    _ => a
                }
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