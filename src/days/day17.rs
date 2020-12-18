use std::fs;

trait Cubes<P> {
    fn new(data: Vec<isize>, size: P) -> Self;

    fn is_valid(&self, pos: &P) -> bool;

    fn index(&self, pos: &P) -> usize;

    fn get_cube(&self, pos: &P) -> isize {
        if self.is_valid(&pos) {
            let index = self.index(&pos);
            self.data()[index]
        } else {
            0
        }
    }

    fn set_cube(&mut self, pos: &P, cube: isize) {
        let index = self.index(&pos);
        self.data_mut()[index] = cube;
    }

    fn count_neighbours(&self, center: &P) -> isize;

    fn evolve(&self, center: &P) -> isize {
        let cube = self.get_cube(&center);
        let count = self.count_neighbours(&center);
        if count == 3 || (count == 2 && cube == 1) { 1 } else { 0 }
    }

    fn cycle(&mut self);

    fn count_active(&self) -> isize {
        self.data().iter().sum()
    }

    fn data(&self) -> &Vec<isize>;

    fn data_mut(&mut self) -> &mut Vec<isize>;
}

type Pos3 = (isize, isize, isize);

struct Cubes3 {
    size: Pos3,
    data: Vec<isize>
}

impl Cubes<Pos3> for Cubes3 {
    fn new(data: Vec<isize>, size: Pos3) -> Self {
        Self { size, data }
    }

    fn is_valid(&self, pos: &Pos3) -> bool {
        (0..self.size.0).contains(&pos.0) &&
            (0..self.size.1).contains(&pos.1) &&
            (0..self.size.2).contains(&pos.2)
    }

    fn index(&self, pos: &Pos3) -> usize {
        (pos.0 + self.size.0 * (pos.1 + self.size.1 * pos.2)) as usize
    }

    fn count_neighbours(&self, center: &Pos3) -> isize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let pos = (center.0 + i, center.1 + j, center.2 + k);
                    if (i, j, k) != (0, 0, 0) && self.is_valid(&pos) {
                        count += self.get_cube(&pos)
                    }
                }
            }
        }
        count
    }

    fn cycle(&mut self) {
        let next_size = (self.size.0 + 2, self.size.1 + 2, self.size.2 + 2);
        let new_data = vec![0; (next_size.0 * next_size.1 * next_size.2) as usize];
        let mut next = Cubes3::new(new_data, next_size);
        for i in 0..next.size.0 {
            for j in 0..next.size.1 {
                for k in 0..next.size.2 {
                    let next_cube = self.evolve(&(i - 1, j - 1, k - 1));
                    next.set_cube(&(i, j, k), next_cube);
                }
            }
        }
        self.size = next.size;
        self.data = next.data;
    }

    fn data(&self) -> &Vec<isize> { &self.data }

    fn data_mut(&mut self) -> &mut Vec<isize> { &mut self.data }
}

type Pos4 = (isize, isize, isize, isize);

struct Cubes4 {
    size: Pos4,
    data: Vec<isize>
}

impl Cubes<Pos4> for Cubes4 {
    fn new(data: Vec<isize>, size: Pos4) -> Self {
        Self { size, data }
    }

    fn is_valid(&self, pos: &Pos4) -> bool {
        (0..self.size.0).contains(&pos.0) &&
            (0..self.size.1).contains(&pos.1) &&
            (0..self.size.2).contains(&pos.2) &&
            (0..self.size.3).contains(&pos.3)
    }

    fn index(&self, pos: &Pos4) -> usize {
        (pos.0 + self.size.0 * (pos.1 + self.size.1 * (pos.2 + self.size.2 * pos.3))) as usize
    }

    fn count_neighbours(&self, center: &Pos4) -> isize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        let pos = (center.0 + i, center.1 + j, center.2 + k, center.3 + l);
                        if (i, j, k, l) != (0, 0, 0, 0) && self.is_valid(&pos) {
                            count += self.get_cube(&pos)
                        }
                    }
                }
            }
        }
        count
    }

    fn cycle(&mut self) {
        let next_size = (self.size.0 + 2, self.size.1 + 2, self.size.2 + 2, self.size.3 + 2);
        let new_data = vec![0; (next_size.0 * next_size.1 * next_size.2 * next_size.3) as usize];
        let mut next = Cubes4::new(new_data, next_size);
        for i in 0..next.size.0 {
            for j in 0..next.size.1 {
                for k in 0..next.size.2 {
                    for l in 0..next.size.3 {
                        let next_cube = self.evolve(&(i - 1, j - 1, k - 1, l - 1));
                        next.set_cube(&(i, j, k, l), next_cube);
                    }
                }
            }
        }
        self.size = next.size;
        self.data = next.data;
    }

    fn data(&self) -> &Vec<isize> { &self.data }

    fn data_mut(&mut self) -> &mut Vec<isize> { &mut self.data }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day17.txt")
        .expect("Something went wrong reading the file");

    let cubes = impl_first_star(&contents);

    println!("day 17.1 - number of active cubes after 6 cycles (3D): {}", cubes);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day17.txt")
        .expect("Something went wrong reading the file");

    let cubes = impl_second_star(&contents);

    println!("day 17.2 - number of active cubes after 6 cycles (4D): {}", cubes);
}

fn impl_first_star(contents: &str) -> isize {
    let (data, width, height) = parse_cubes(contents);
    let cubes = Cubes3::new(data, (width, height, 1));
    run_cycles(cubes)
}

fn impl_second_star(contents: &str) -> isize {
    let (data, width, height) = parse_cubes(contents);
    let cubes = Cubes4::new(data, (width, height, 1, 1));
    run_cycles(cubes)
}

fn run_cycles<P, C>(mut cubes: C) -> isize where C: Cubes<P> {
    for _ in 0..6 {
        cubes.cycle();
    }
    cubes.count_active()
}

fn parse_cubes(contents: &str) -> (Vec<isize>, isize, isize) {
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().len();
    let cubes = contents
        .chars()
        .filter_map(|c|
            match c {
                '.' => Some(0),
                '#' => Some(1),
                _ => None
            }
        )
        .collect::<Vec<_>>();
    (cubes, width as isize, height as isize)
}

#[test]
fn test0_first_star() {
    let contents =
        ".#.\n\
         ..#\n\
         ###";
    assert_eq!(impl_first_star(contents), 112);
}

#[test]
fn test0_second_star() {
    let contents =
        ".#.\n\
         ..#\n\
         ###";
    assert_eq!(impl_second_star(contents), 848);
}
