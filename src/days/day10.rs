use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day10.txt")
        .expect("Something went wrong reading the file");

    let diff = impl_first_star(&contents);

    println!("day 10.1 - 1-jolt differences times 3-jolt differences: {}", diff);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day10.txt")
        .expect("Something went wrong reading the file");

    let arrangements = impl_second_star(&contents);

    println!("day 10.2 - total number of arrangements: {}", arrangements);
}

fn impl_first_star(contents: &str) -> usize {
    let jolts = parse_jolts(contents);
    let diff = jolts
        .windows(2)
        .fold((0, 0), |sum, pair| {
            match  pair[1] - pair[0] {
                1 => (sum.0 + 1, sum.1),
                3 => (sum.0, sum.1 + 1),
                _ => sum
            }
        });
    diff.0 * diff.1
}

fn impl_second_star(contents: &str) -> usize {
    let jolts = parse_jolts(contents);
    let mut arr = Vec::<usize>::new();
    arr.push(1);
    for i in 1..jolts.len() {
        let mut total = 0;
        for d in 1..4 {
            if d <= i && jolts[i] - jolts[i - d] <= 3 {
                total += arr[i - d];
            }
        }
        arr.push(total);
    }
    *arr.last().unwrap()
}

fn parse_jolts(contents: &str) -> Vec<usize> {
    let mut jolts = contents
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    jolts.push(0);
    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);
    jolts
}

#[test]
fn test0_first_star() {
    let contents =
        "16\n\
         10\n\
         15\n\
         5\n\
         1\n\
         11\n\
         7\n\
         19\n\
         6\n\
         12\n\
         4";
    assert_eq!(impl_first_star(contents), 35);
}

#[test]
fn test1_first_star() {
    let contents =
        "28\n\
         33\n\
         18\n\
         42\n\
         31\n\
         14\n\
         46\n\
         20\n\
         48\n\
         47\n\
         24\n\
         23\n\
         49\n\
         45\n\
         19\n\
         38\n\
         39\n\
         11\n\
         1\n\
         32\n\
         25\n\
         35\n\
         8\n\
         17\n\
         7\n\
         9\n\
         4\n\
         2\n\
         34\n\
         10\n\
         3";
    assert_eq!(impl_first_star(contents), 220);
}

#[test]
fn test0_second_star() {
    let contents =
        "16\n\
         10\n\
         15\n\
         5\n\
         1\n\
         11\n\
         7\n\
         19\n\
         6\n\
         12\n\
         4";
    assert_eq!(impl_second_star(contents), 8);
}

#[test]
fn test1_second_star() {
    let contents =
        "28\n\
         33\n\
         18\n\
         42\n\
         31\n\
         14\n\
         46\n\
         20\n\
         48\n\
         47\n\
         24\n\
         23\n\
         49\n\
         45\n\
         19\n\
         38\n\
         39\n\
         11\n\
         1\n\
         32\n\
         25\n\
         35\n\
         8\n\
         17\n\
         7\n\
         9\n\
         4\n\
         2\n\
         34\n\
         10\n\
         3";
    assert_eq!(impl_second_star(contents), 19208);
}
