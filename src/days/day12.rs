use std::fs;

struct Ship {
    pos: (isize, isize),
    dir: (isize, isize),
    waypoint: bool
}

impl Ship {
    fn new(dir: (isize, isize), waypoint: bool) -> Self { Self { pos: (0, 0), dir, waypoint } }

    fn next(&mut self, instr: (char, isize)) {
        match instr {
            ('L', 180) | ('R', 180) => self.dir = (-self.dir.0, -self.dir.1),
            ('L', 90)  | ('R', 270) => self.dir = (-self.dir.1,  self.dir.0),
            ('L', 270) | ('R', 90)  => self.dir = ( self.dir.1, -self.dir.0),
            ('N', value) if self.waypoint  => self.dir = (self.dir.0, self.dir.1 + value),
            ('N', value) if !self.waypoint => self.pos = (self.pos.0, self.pos.1 + value),
            ('S', value) if self.waypoint  => self.dir = (self.dir.0, self.dir.1 - value),
            ('S', value) if !self.waypoint => self.pos = (self.pos.0, self.pos.1 - value),
            ('E', value) if self.waypoint  => self.dir = (self.dir.0 + value, self.dir.1),
            ('E', value) if !self.waypoint => self.pos = (self.pos.0 + value, self.pos.1),
            ('W', value) if self.waypoint  => self.dir = (self.dir.0 - value, self.dir.1),
            ('W', value) if !self.waypoint => self.pos = (self.pos.0 - value, self.pos.1),
            ('F', value) => {
                self.pos.0 += self.dir.0 * value;
                self.pos.1 += self.dir.1 * value;
            }
            _ => panic!("invalid instruction")
        }
    }

    fn navigate(&mut self, instructions: &[(char, isize)]) {
        instructions.iter().for_each(|instr| self.next(*instr));
    }

    fn manhattan_distance(self) -> isize {
        isize::abs(self.pos.0) + isize::abs(self.pos.1)
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day12.txt")
        .expect("Something went wrong reading the file");

    let distance = impl_first_star(&contents);

    println!("day 12.1 - manhattan distance: {}", distance);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day12.txt")
        .expect("Something went wrong reading the file");

    let distance = impl_second_star(&contents);

    println!("day 12.2 - manhattan distance with waypoint: {}", distance);
}

fn impl_first_star(contents: &str) -> isize {
    let instructions = parse_instructions(contents);
    let mut ship = Ship::new((1, 0), false);
    ship.navigate(&instructions);
    ship.manhattan_distance()
}

fn impl_second_star(contents: &str) -> isize {
    let instructions = parse_instructions(contents);
    let mut ship = Ship::new((10, 1), true);
    ship.navigate(&instructions);
    ship.manhattan_distance()
}

fn parse_instructions(contents: &str) -> Vec<(char, isize)> {
    contents
        .lines()
        .map(|line| {
            let instr = line.chars().next().unwrap();
            let value = line[1..].parse::<isize>().unwrap();
            (instr, value)
        })
        .collect()
}

#[test]
fn test0_first_star() {
    let contents =
        "F10\n\
         N3\n\
         F7\n\
         R90\n\
         F11";
    assert_eq!(impl_first_star(contents), 25);
}

#[test]
fn test0_second_star() {
    let contents =
        "F10\n\
         N3\n\
         F7\n\
         R90\n\
         F11";
    assert_eq!(impl_second_star(contents), 286);
}