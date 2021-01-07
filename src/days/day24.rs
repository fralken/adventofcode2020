use std::fs;
use std::collections::HashSet;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day24.txt")
        .expect("Something went wrong reading the file");

    let tiles = impl_first_star(&contents);

    println!("day 24.1 - number of black tiles: {}", tiles);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day24.txt")
        .expect("Something went wrong reading the file");

    let tiles = impl_second_star(&contents, 100);

    println!("day 24.2 - number of black tiles after 100 days: {}", tiles);
}

fn impl_first_star(contents: &str) -> usize {
    let paths = parse(contents);
    black_tiles(&paths).len()
}

fn impl_second_star(contents: &str, days: usize) -> usize {
    let paths = parse(contents);
    let mut black_tiles = black_tiles(&paths);
    for _ in 1..=days {
        black_tiles = black_tiles.iter()
            .flat_map(neighbours)
            .chain(black_tiles.iter().cloned())
            .filter(|tile| {
                let black_neighbours = neighbours(&tile)
                    .filter(|neighbour| black_tiles.contains(neighbour))
                    .count();
                black_neighbours == 2 ||
                    (black_tiles.contains(&tile) &&
                        black_neighbours > 0 && black_neighbours <= 2)
            })
            .collect();
    }
    black_tiles.len()
}

type Pos = (i32, i32);

fn parse(contents: &str) -> Vec<Vec<String>> {
    contents.lines()
        .map(|line| {
            let mut steps = Vec::new();
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                match c {
                    'e' | 'w' => steps.push(c.to_string()),
                    's' | 'n' => steps.push(format!("{}{}", c, chars.next().unwrap())),
                    _ => panic!("unrecognized character {}", c)
                }
            }
            steps
        })
        .collect()
}

fn black_tiles(paths: &[Vec<String>]) -> HashSet<Pos> {
    paths.iter()
        .map(|path| go(path))
        .fold(HashSet::new(), |mut set, tile| {
            if !set.insert(tile) { set.remove(&tile); };
            set
        })
}

fn go(path: &[String]) -> Pos {
    path.iter().fold((0, 0), |pos, dir| coord(&pos, dir))
}

fn coord(pos: &Pos, dir: &str) -> Pos {
    match dir {
        "e" => (pos.0 + 1, pos.1),
        "w" => (pos.0 - 1, pos.1),
        "ne" => (pos.0, pos.1 + 1),
        "nw" => (pos.0 - 1, pos.1 + 1),
        "se" => (pos.0 + 1, pos.1 - 1),
        "sw" => (pos.0, pos.1 - 1),
        _ => panic!("invalid direction: {}", dir)
    }
}

fn neighbours(pos: &Pos) -> impl Iterator<Item=Pos> + '_ {
    ["e", "w", "ne", "nw", "se", "sw"].iter()
        .map(move |dir| coord(pos, dir))
}

#[test]
fn test0_first_star() {
    let contents = "esenee";
    let paths = parse(contents);
    assert_eq!(go(paths.first().unwrap()), (3, 0));
}

#[test]
fn test1_first_star() {
    let contents = "esew";
    let paths = parse(contents);
    assert_eq!(go(paths.first().unwrap()), (1, -1));
}

#[test]
fn test2_first_star() {
    let contents = "nwwswee";
    let paths = parse(contents);
    assert_eq!(go(paths.first().unwrap()), (0, 0));
}

#[test]
fn test3_first_star() {
    let contents = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    assert_eq!(impl_first_star(contents), 10);
}

#[test]
fn test0_second_star() {
    let contents = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    assert_eq!(impl_second_star(contents, 100), 2208);
}