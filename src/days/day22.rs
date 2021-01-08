use std::fs;
use std::collections::VecDeque;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day22.txt")
        .expect("Something went wrong reading the file");

    let score = impl_first_star(&contents);

    println!("day 22.1 - winning player's score: {}", score);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day22.txt")
        .expect("Something went wrong reading the file");

    let score = impl_second_star(&contents);

    println!("day 22.2 - winning player's score (recursive game): {}", score);
}

fn impl_first_star(contents: &str) -> usize {
    let players = parse_cards(contents);
    play(&players[0], &players[1], check_single).1
}

fn impl_second_star(contents: &str) -> usize {
    let players = parse_cards(contents);
    play(&players[0], &players[1], check_recursive).1
}

fn parse_cards(contents: &str) -> Vec<VecDeque<usize>> {
    contents.split("\n\n")
        .map(|player|
            player.lines()
                .skip(1)
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<VecDeque<_>>()
        )
        .collect::<Vec<_>>()
}

fn play<F>(p1_start: &VecDeque<usize>, p2_start: &VecDeque<usize>, check: F) -> (bool, usize)
    where F: Fn(usize, usize, &VecDeque<usize>, &VecDeque<usize>) -> bool {
    let mut p1 = p1_start.clone();
    let mut p2 = p2_start.clone();
    let mut p1_history = vec![p1_start.clone()];
    let mut p2_history = vec![p2_start.clone()];
    let mut infinite = false;
    while !p1.is_empty() && !p2.is_empty() && !infinite {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if check(c1, c2, &p1, &p2) {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
        infinite = p1_history.contains(&p1) && p2_history.contains(&p2);
        if !infinite {
            p1_history.push(p1.clone());
            p2_history.push(p2.clone());
        };
    }
    let winner_p1 = p2.is_empty() || infinite;
    let winner_cards = if winner_p1 { p1 } else { p2 };
    (winner_p1, winner_cards.iter().enumerate().map(|(i, c)| (winner_cards.len() - i) * c).sum())
}

fn check_single(c1: usize, c2: usize, _p1: &VecDeque<usize>, _p2: &VecDeque<usize>) -> bool {
    c1 > c2
}

fn check_recursive(c1: usize, c2: usize, p1: &VecDeque<usize>, p2: &VecDeque<usize>) -> bool {
    if p1.len() >= c1 && p2.len() >= c2 {
        play(&p1.iter().take(c1).cloned().collect(),
             &p2.iter().take(c2).cloned().collect(), check_recursive).0
    } else {
        c1 > c2
    }
}

#[test]
fn test0_first_star() {
    let contents =
        "Player 1:\n\
         9\n\
         2\n\
         6\n\
         3\n\
         1\n\
         \n\
         Player 2:\n\
         5\n\
         8\n\
         4\n\
         7\n\
         10";
    assert_eq!(impl_first_star(contents), 306);
}

#[test]
fn test0_second_star() {
    let contents =
        "Player 1:\n\
         9\n\
         2\n\
         6\n\
         3\n\
         1\n\
         \n\
         Player 2:\n\
         5\n\
         8\n\
         4\n\
         7\n\
         10";
    assert_eq!(impl_second_star(contents), 291);
}

#[test]
fn test1_second_star() {
    let contents =
        "Player 1:\n\
         43\n\
         19\n\
         \n\
         Player 2:\n\
         2\n\
         29\n\
         14";
    let players = parse_cards(contents);
    assert_eq!(play(&players[0], &players[1], check_recursive).0, true);
}
