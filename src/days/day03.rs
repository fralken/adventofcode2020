use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day03.txt")
        .expect("Something went wrong reading the file");

    let trees = impl_first_star(&contents, 3, 1);

    println!("day  3.1 - number of trees for slop right 3 down 1: {}", trees);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day03.txt")
        .expect("Something went wrong reading the file");

    let trees = impl_second_star(&contents, &[(1, 1), (3, 1), (5, 1), (7, 1), (1 ,2)]);

    println!("day  3.2 - product of trees for list of slopes: {}", trees);
}

fn impl_first_star(contents: &str, right: usize, down: usize) -> usize {
    let field = parse_field(&contents);

    trees_for_slope(&field, right, down)
}

fn impl_second_star(contents: &str, slopes: &[(usize, usize)]) -> usize {
    let field = parse_field(&contents);

    slopes.iter()
        .map(|slope| trees_for_slope(&field, slope.0, slope.1))
        .product()
}

fn parse_field(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn trees_for_slope(field: &[Vec<char>], right: usize, down: usize) -> usize {
    let mut x = right;
    let mut y = down;
    let mut trees = 0;
    while y < field.len() {
        if field[y][x] == '#' { trees += 1; }
        x = (x + right) % field[y].len();
        y += down;
    }
    trees
}

#[test]
fn test0_first_star() {
    let field =
        "..##.......\n\
         #...#...#..\n\
         .#....#..#.\n\
         ..#.#...#.#\n\
         .#...##..#.\n\
         ..#.##.....\n\
         .#.#.#....#\n\
         .#........#\n\
         #.##...#...\n\
         #...##....#\n\
         .#..#...#.#";
    assert_eq!(impl_first_star(field, 3, 1), 7);
}

#[test]
fn test0_second_star() {
    let field =
        "..##.......\n\
         #...#...#..\n\
         .#....#..#.\n\
         ..#.#...#.#\n\
         .#...##..#.\n\
         ..#.##.....\n\
         .#.#.#....#\n\
         .#........#\n\
         #.##...#...\n\
         #...##....#\n\
         .#..#...#.#";
    assert_eq!(impl_second_star(field, &[(1, 1), (3, 1), (5, 1), (7, 1), (1 ,2)]), 336);
}
