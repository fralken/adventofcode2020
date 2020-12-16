use std::fs;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use text_io::scan;

#[derive(Debug)]
struct Field {
    name: String,
    ranges: [RangeInclusive<usize>; 2],
}

impl Field {
    fn new(name: String, min1: usize, max1: usize, min2: usize, max2: usize) -> Self {
        Field { name, ranges: [(min1..=max1), (min2..=max2)] }
    }

    fn is_valid(&self, num: &usize) -> bool {
        self.ranges.iter().any(|r| r.contains(num))
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day16.txt")
        .expect("Something went wrong reading the file");

    let rate = impl_first_star(&contents);

    println!("day 16.1 - ticket scanning error rate: {}", rate);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day16.txt")
        .expect("Something went wrong reading the file");

    let num = impl_second_star(&contents);

    println!("day 16.2 - product of departure fields: {}", num);
}

fn impl_first_star(contents: &str) -> usize {
    let (fields, _, tickets) = parse_tickets(contents);

    tickets.iter()
        .map(|t|
            t.iter()
                .filter(|n| fields.iter().all(|f| !f.is_valid(n)))
                .sum::<usize>()
        )
        .sum()
}

fn impl_second_star(contents: &str) -> usize {
    let (fields, ticket, tickets) = parse_tickets(contents);

    let mut valid_tickets = tickets.into_iter()
        .filter(|t|
            t.iter()
                .all(|n| fields.iter().any(|f| f.is_valid(n)))
        )
        .collect::<Vec<_>>();

    valid_tickets.push(ticket.clone());
    let ticket_len = valid_tickets[0].len();

    let mut valid_positions_sets = fields.iter()
        .map(|field| {
            let mut valid = HashSet::new();
            for i in 0..ticket_len {
                let mut all = true;
                for t in &valid_tickets {
                    all = all && field.is_valid(&t[i]);
                    if !all { break }
                }
                if all { valid.insert(i); };
            }
            valid
        })
        .enumerate()
        .collect::<Vec<_>>();

    fn sort(positions: &mut Vec<(usize, HashSet<usize>)>) {
        positions.sort_by(|a, b|
            b.1.len().partial_cmp(&a.1.len()).unwrap());
    }

    sort(&mut valid_positions_sets);
    let mut valid_positions = vec![0; ticket.len()];
    while !valid_positions_sets.is_empty() {
        let (i, set) = valid_positions_sets.pop().unwrap();
        let pos = set.iter().next().unwrap();
        valid_positions_sets.iter_mut().for_each(|(_, v)| { v.remove(pos); });
        sort(&mut valid_positions_sets);
        valid_positions[i] = *pos;
    }

    fields.iter()
        .zip(valid_positions.iter())
        .filter_map(|(field, pos)|
            if field.name.starts_with("departure") {
                Some(ticket[*pos])
            } else {
                None
            }
        )
        .product()
}

fn parse_tickets(contents: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>){
    let mut blocks = contents.split("\n\n");

    let fields = blocks
        .next().unwrap()
        .lines()
        .map(|line| {
            let name: String;
            let min1: usize;
            let max1: usize;
            let min2: usize;
            let max2: usize;
            scan!(line.bytes() => "{}: {}-{} or {}-{}", name, min1, max1, min2, max2);
            Field::new(name, min1, max1, min2, max2)
        }).collect::<Vec<_>>();

    let ticket = blocks
        .next().unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .last()
        .unwrap();

    let tickets = blocks
        .next().unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    (fields, ticket, tickets)
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

#[test]
fn test0_first_star() {
    let contents =
    "class: 1-3 or 5-7\n\
     row: 6-11 or 33-44\n\
     seat: 13-40 or 45-50\n\
     \n\
     your ticket:\n\
     7,1,14\n\
     \n\
     nearby tickets:\n\
     7,3,47\n\
     40,4,50\n\
     55,2,20\n\
     38,6,12";
    assert_eq!(impl_first_star(contents), 71);
}
