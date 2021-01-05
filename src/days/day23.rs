use std::fs;

struct Cups {
    first: usize,
    cups: Vec<usize>
}

impl Cups {
    fn parse(contents: &str, extend_to: Option<usize>) -> Self {
        let input = contents.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let &first = input.first().unwrap();
        let &last = input.last().unwrap();
        let mut cups = vec![0; input.len() + 1];
        input.iter().skip(1).fold(first, |prev, &next| {
            cups[prev] = next;
            next
        });
        if let Some(limit) = extend_to {
            cups[last] = cups.len();
            for i in (cups[last] + 1)..=limit {
                cups.push(i);
            }
            cups.push(first);
        } else {
            cups[last] = first;
        }
        Cups { first, cups }
    }

    fn next_after(&self, step: usize) -> usize {
        let mut curr= self.first;
        for _ in 0..step {
            curr = self.cups[curr];
        }
        curr
    }

    fn destination(&self, after: usize) -> usize {
        let mut dest = self.decr(self.first);
        let mut curr = self.cups[self.first];
        while curr != self.cups[after] {
            if dest == curr {
                dest = self.decr(dest);
                curr = self.cups[self.first];
            }
            else {
                curr = self.cups[curr];
            }
        }
        dest
    }

    fn decr(&self, curr: usize) -> usize {
        let curr = (curr as isize - 1) as usize;
        if curr == 0 { self.cups.len() - 1 } else { curr }
    }

    fn move_slice(&mut self, next: usize, dest: usize) {
        let next_dest = self.cups[dest];
        self.cups[dest] = self.cups[self.first];
        self.cups[self.first] = self.cups[next];
        self.cups[next] = next_dest;
    }

    fn move_top(&mut self) {
        self.first = self.cups[self.first];
    }

    fn turn(&mut self) {
        let next = self.next_after(3);
        let dest = self.destination(next);
        self.move_slice(next, dest);
        self.move_top()
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day23.txt")
        .expect("Something went wrong reading the file");

    let res = impl_first_star(&contents, 100);

    println!("day 23.1 - labels on cups after cup 1: {}", res);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day23.txt")
        .expect("Something went wrong reading the file");

    let res = impl_second_star(&contents);

    println!("day 23.2 - two cups after cup 1 multiplied together: {}", res);
}

fn impl_first_star(contents: &str, times: usize) -> usize {
    let cups = play(&contents, times, None);
    let mut seq = 0;
    let mut curr = cups.cups[1];
    while curr != 1 {
        seq = seq * 10 + curr;
        curr = cups.cups[curr];
    }
    seq
}

fn impl_second_star(contents: &str) -> usize {
    let cups = play(&contents, 10_000_000, Some(1_000_000));
    let c1 = cups.cups[1];
    let c2 = cups.cups[c1];
    c1 * c2
}

fn play(contents: &str, times: usize, extend_to: Option<usize>) -> Cups {
    let mut cups = Cups::parse(&contents, extend_to);
    for _ in 0..times {
        cups.turn();
    }
    cups
}

#[test]
fn test0_first_star() {
    let contents = "389125467";
    assert_eq!(impl_first_star(contents, 10), 92658374);
}

#[test]
fn test1_first_star() {
    let contents = "389125467";
    assert_eq!(impl_first_star(contents, 100), 67384529);
}

#[test]
fn test0_second_star() {
    let contents = "389125467";
    assert_eq!(impl_second_star(contents), 149245887792);
}
