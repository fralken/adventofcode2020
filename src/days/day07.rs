use std::fs;
use std::collections::{ HashMap, HashSet };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day07.txt")
        .expect("Something went wrong reading the file");

    let counts = impl_first_star(&contents, "shiny gold");

    println!("day  7.1 - number of bag colors containing 1 shiny gold bag: {}", counts);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day07.txt")
        .expect("Something went wrong reading the file");

    let counts = impl_second_star(&contents, "shiny gold");

    println!("day  7.2 - number of bags contained in 1 shiny gold bag: {}", counts);
}

fn impl_first_star(contents: &str, bag_color: &str) -> usize {
    let bags = parse_bags(contents);
    let mut contained = Vec::new();
    let mut containing = HashSet::new();
    contained.push(bag_color);
    while !contained.is_empty() {
        let bag = contained.pop().unwrap();
        bags
            .iter()
            .for_each(|b|
                if b.1.contains_key(bag) {
                    contained.push(b.0);
                    containing.insert(b.0);
                }
            )
    }

    containing.len()
}

fn impl_second_star(contents: &str, bag_color: &str) -> usize {
    let bags = parse_bags(contents);
    let mut containing = Vec::new();
    let mut count = 0;
    containing.push((bag_color, 1));
    while !containing.is_empty() {
        let (color, quantity) = containing.pop().unwrap();
        if let Some(contained) = bags.get(color) {
            contained
                .iter()
                .for_each(|(c, q)| {
                    let qq = q * quantity;
                    count += qq;
                    containing.push((c, qq))
                });
        }
    }

    count
}

fn parse_bags(contents: &str) -> HashMap<String, HashMap<String, usize>> {
    contents
        .lines()
        .map(|line| {
            let mut split = line.splitn(2, "contain");
            let container_color = split
                .next()
                .unwrap()
                .split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .join(" ");
            let contained_colors = split
                .next()
                .unwrap()
                .split(',')
                .filter_map(|s| {
                    let c = s.split_whitespace().collect::<Vec<_>>();
                    if let Ok(q) = c[0].parse::<usize>() {
                        Some((format!("{} {}", c[1], c[2]), q))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>();
            (container_color, contained_colors)
        }).collect()
}

#[test]
fn test0_first_star() {
    let contents =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
         dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
         bright white bags contain 1 shiny gold bag.\n\
         muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
         shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
         dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
         vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
         faded blue bags contain no other bags.\n\
         dotted black bags contain no other bags.";
    assert_eq!(impl_first_star(contents, "shiny gold"), 4);
}

#[test]
fn test0_second_star() {
    let contents =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
         dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
         bright white bags contain 1 shiny gold bag.\n\
         muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
         shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
         dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
         vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
         faded blue bags contain no other bags.\n\
         dotted black bags contain no other bags.";
    assert_eq!(impl_second_star(contents, "shiny gold"), 32);
}

#[test]
fn test1_second_star() {
    let contents =
        "shiny gold bags contain 2 dark red bags.\n\
         dark red bags contain 2 dark orange bags.\n\
         dark orange bags contain 2 dark yellow bags.\n\
         dark yellow bags contain 2 dark green bags.\n\
         dark green bags contain 2 dark blue bags.\n\
         dark blue bags contain 2 dark violet bags.\n\
         dark violet bags contain no other bags.";
    assert_eq!(impl_second_star(contents, "shiny gold"), 126);
}
