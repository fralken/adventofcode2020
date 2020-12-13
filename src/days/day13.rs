use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day13.txt")
        .expect("Something went wrong reading the file");

    let result = impl_first_star(&contents);

    println!("day 13.1 - bus ID multiplied minutes to wait: {}", result);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day13.txt")
        .expect("Something went wrong reading the file");

    let result = impl_second_star(&contents);

    println!("day 13.2 - earliest timestamp: {}", result);
}

fn impl_first_star(contents: &str) -> usize {
    let (start, ids) = parse_schedules(contents);
    let (time, id) = ids.iter()
        .filter_map(|id| id.parse::<usize>().ok())
        .map(|id| (id - start % id, id))
        .min()
        .unwrap();
    time * id
}

fn impl_second_star(contents: &str) -> usize {
    let (_, ids) = parse_schedules(contents);
    let (_, time) = ids.iter()
        .enumerate()
        .filter_map(|(i, s)|
            if let Ok(id) = s.parse::<usize>() {
                Some((i, id))
            } else {
                None
            }
        )
        .fold((1, 0), |(p, mut t), (i, id)| {
            while (i + t) % id != 0 { t += p; }
            (p * id, t)
        });
    time
}

fn parse_schedules(contents: &str) -> (usize, Vec<&str>) {
    let mut lines = contents.lines();
    let start = lines.next().unwrap()
        .parse::<usize>().unwrap();
    let ids = lines.next().unwrap()
        .split(',')
        .collect();
    (start, ids)
}

#[test]
fn test0_first_star() {
    let contents = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(impl_first_star(contents), 295);
}

#[test]
fn test0_second_star() {
    let contents = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(impl_second_star(contents), 1068781);
}

#[test]
fn test1_second_star() {
    let contents = "0\n17,x,13,19";
    assert_eq!(impl_second_star(contents), 3417);
}

#[test]
fn test2_second_star() {
    let contents = "0\n67,7,59,61";
    assert_eq!(impl_second_star(contents), 754018);
}

#[test]
fn test3_second_star() {
    let contents = "0\n67,x,7,59,61";
    assert_eq!(impl_second_star(contents), 779210);
}

#[test]
fn test4_second_star() {
    let contents = "0\n67,7,x,59,61";
    assert_eq!(impl_second_star(contents), 1261476);
}

#[test]
fn test5_second_star() {
    let contents = "0\n1789,37,47,1889";
    assert_eq!(impl_second_star(contents), 1202161486);
}
