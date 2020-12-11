use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day11.txt")
        .expect("Something went wrong reading the file");

    let seats = impl_first_star(&contents);

    println!("day 11.1 - seats occupied after stability (adjacent rule): {}", seats);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day11.txt")
        .expect("Something went wrong reading the file");

    let seats = impl_second_star(&contents);

    println!("day 11.2 - seats occupied after stability (visible rule): {}", seats);
}

fn impl_first_star(contents: &str) -> usize {
    count_after_stability(contents, count_adjacent)
}

fn impl_second_star(contents: &str) -> usize {
    count_after_stability(contents, count_visible)
}

fn count_after_stability<F>(contents: &str, counter: F) -> usize
    where F: Fn(&[Vec<char>], usize, usize) -> char {
    let mut prev = parse_seats(contents);
    loop {
        let (next, changed) = step(&prev, &counter);
        if !changed {
            return next
                .iter()
                .map(|row|
                    row.iter()
                        .filter(|&c| *c == '#')
                        .count())
                .sum()
        }
        prev = next
    }
}

fn parse_seats(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn step<F>(prev: &[Vec<char>], counter: F) -> (Vec<Vec<char>>, bool)
    where F: Fn(&[Vec<char>], usize, usize) -> char {
    let height = prev.len();
    let width = prev[0].len();
    let mut next = Vec::new();
    let mut changed = false;
    for y in 0..height {
        next.push(Vec::new());
        for x in 0..width {
            let seat = counter(&prev, x, y);
            if prev[y][x] != seat { changed = true }
            next[y].push(seat);
        }
    }
    (next, changed)
}

fn count_adjacent(room: &[Vec<char>], x: usize, y: usize) -> char {
    let height = room.len()-2;
    let width = room[0].len()-2;
    let mut seat = room[y][x];
    if seat != '.' {
        let mut adjacent = 0;
        for i in x.max(1)-1..=x.min(width)+1 {
            for j in y.max(1)-1..=y.min(height)+1 {
                if (i, j) != (x, y) && room[j][i] == '#' { adjacent += 1 }
            }
        }
        if adjacent == 0 {
            seat = '#';
        } else if adjacent >= 4 {
            seat = 'L';
        }
    }
    seat
}

fn count_visible(room: &[Vec<char>], x: usize, y: usize) -> char {
    let height = room.len();
    let width = room[0].len();
    let mut seat = room[y][x];
    if seat != '.' {
        let mut visible = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if (i, j) != (0, 0) {
                    let (mut r, mut c) = (y as isize + j, x as isize + i);
                    while r >= 0 && r < height as isize && c >= 0 && c < width as isize {
                        match room[r as usize][c as usize] {
                            '#' => { visible += 1; break; },
                            'L' => break,
                            _ => { r += j; c += i; }
                        }
                    }
                }
            }
        }
        if visible == 0 {
            seat = '#';
        } else if visible >= 5 {
            seat = 'L';
        }
    }
    seat
}

#[test]
fn test0_first_star() {
    let contents =
        "L.LL.LL.LL\n\
         LLLLLLL.LL\n\
         L.L.L..L..\n\
         LLLL.LL.LL\n\
         L.LL.LL.LL\n\
         L.LLLLL.LL\n\
         ..L.L.....\n\
         LLLLLLLLLL\n\
         L.LLLLLL.L\n\
         L.LLLLL.LL";
    assert_eq!(impl_first_star(contents), 37);
}

#[test]
fn test0_second_star() {
    let contents =
        "L.LL.LL.LL\n\
         LLLLLLL.LL\n\
         L.L.L..L..\n\
         LLLL.LL.LL\n\
         L.LL.LL.LL\n\
         L.LLLLL.LL\n\
         ..L.L.....\n\
         LLLLLLLLLL\n\
         L.LLLLLL.L\n\
         L.LLLLL.LL";
    assert_eq!(impl_second_star(contents), 26);
}