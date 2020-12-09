use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day09.txt")
        .expect("Something went wrong reading the file");

    let code = impl_first_star(&contents, 25);

    println!("day  9.1 - first invalid number: {}", code);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day09.txt")
        .expect("Something went wrong reading the file");

    let weakness = impl_second_star(&contents, 25).unwrap();

    println!("day  9.2 - encryption weakness: {}", weakness);
}

fn impl_first_star(contents: &str, batch: usize) -> usize {
    let codes = parse(contents);
    first_invalid_code(&codes, batch)
}

fn impl_second_star(contents: &str, batch: usize) -> Option<usize> {
    let codes = parse(contents);
    let invalid = first_invalid_code(&codes, batch);
    for i in 0..codes.len() {
        let mut acc = codes[i];
        let mut j = i + 1;
        while acc < invalid && codes[j] != invalid && j < codes.len() {
            acc += codes[j];
            if acc == invalid {
                let set = &codes[i..=j];
                let min = set.iter().min().unwrap();
                let max = set.iter().max().unwrap();
                return Some(min + max)
            }
            j += 1;
        }
    }
    None
}

fn parse(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn first_invalid_code(codes: &[usize], batch: usize) -> usize {
    *codes.windows(batch)
        .zip(codes[batch..].iter())
        .filter_map(|(window, &code)| {
            for i in window {
                for j in window {
                    if i != j && i + j == code {
                        return None
                    }
                }
            }
            Some(code)
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
}

#[test]
fn test0_first_star() {
    let contents =
        "35\n\
         20\n\
         15\n\
         25\n\
         47\n\
         40\n\
         62\n\
         55\n\
         65\n\
         95\n\
         102\n\
         117\n\
         150\n\
         182\n\
         127\n\
         219\n\
         299\n\
         277\n\
         309\n\
         576";
    assert_eq!(impl_first_star(contents, 5), 127);
}

#[test]
fn test0_second_star() {
    let contents =
        "35\n\
         20\n\
         15\n\
         25\n\
         47\n\
         40\n\
         62\n\
         55\n\
         65\n\
         95\n\
         102\n\
         117\n\
         150\n\
         182\n\
         127\n\
         219\n\
         299\n\
         277\n\
         309\n\
         576";
    assert_eq!(impl_second_star(contents, 5), Some(62));
}