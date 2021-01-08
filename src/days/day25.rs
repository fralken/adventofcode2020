use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day25.txt")
        .expect("Something went wrong reading the file");

    let key = impl_first_star(&contents);

    println!("day 25.1 - encryption key: {}", key);
}

pub fn second_star() {
    println!("day 25.2 - THE END ... Thanks for watching");
}

fn impl_first_star(contents: &str) -> usize {
    let modulo = 20_201_227;
    let subject_number = 7;
    let (card_key, door_key) = parse(contents);
    let card_loop_size = loop_size(subject_number, modulo, card_key);
    let door_loop_size = loop_size(subject_number, modulo, door_key);
    let encryption_key = encrypt(card_key, door_loop_size, modulo);
    assert_eq!(encryption_key, encrypt(door_key, card_loop_size, modulo));
    encryption_key
}

fn parse(contents: &str) -> (usize, usize) {
    let mut keys = contents.lines().flat_map(|line| line.parse::<usize>());
    (keys.next().unwrap(), keys.next().unwrap())
}

fn loop_size(subject_number: usize, modulo: usize, public_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != public_key {
        loop_size += 1;
        value = (value * subject_number) % modulo;
    }
    loop_size
}

fn encrypt(subject_number: usize, loop_size: usize, modulo: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % modulo;
    }
    value
}

#[test]
fn test0_first_star() {
    let contents =
        "5764801\n\
         17807724";
    assert_eq!(impl_first_star(contents), 14897079);
}
