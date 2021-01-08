use std::fs;

struct Cubes {
    size: Vec<isize>,
    data: Vec<isize>
}

impl Cubes {
    fn new(data: Vec<isize>, size: Vec<isize>) -> Self {
        Self { size, data }
    }

    fn is_valid(&self, pos: &[isize]) -> bool {
        self.size.iter()
            .zip(pos.iter())
            .all(|(&s, p)| (0..s).contains(p))
    }

    fn index(&self, pos: &[isize]) -> usize {
        fn ind(s: &[isize], p: &[isize]) -> isize {
            if p.is_empty() { 0 } else { p[0] + s[0] * ind(&s[1..], &p[1..]) }
        }
        ind(&self.size, pos) as usize
    }

    fn get_cube(&self, pos: &[isize]) -> isize {
        if self.is_valid(&pos) {
            let index = self.index(pos);
            self.data[index]
        } else {
            0
        }
    }

    fn set_cube(&mut self, pos: &[isize], cube: isize) {
        let index = self.index(pos);
        self.data[index] = cube;
    }

    fn count_neighbours(&self, center: &[isize]) -> isize {
        fn cn(cube: &Cubes, dim: &[isize], c: &[isize]) -> isize {
            if dim.len() == c.len() {
                let pos = c.iter().zip(dim.iter()).map(|(c, d)| c + d).collect::<Vec<_>>();
                if dim.iter().any(|&d| d != 0) && cube.is_valid(&pos) {
                    cube.get_cube(&pos)
                } else {
                    0
                }
            } else {
                let mut count = 0;
                for i in -1..=1 {
                    count += cn(cube, &[&dim, &[i as isize][..]].concat(), c);
                }
                count
            }
        }
        cn(self, &[], center)
    }

    fn evolve(&self, center: &[isize]) -> isize {
        let cube = self.get_cube(&center);
        let count = self.count_neighbours(&center);
        if count == 3 || (count == 2 && cube == 1) { 1 } else { 0 }
    }

    fn cycle(&mut self) {
        fn c(next: &mut Cubes, cube: &Cubes, dim: &[isize]) {
            if dim.len() == cube.size.len() {
                let next_cube = cube.evolve(&dim.iter().map(|d| d - 1).collect::<Vec<_>>());
                next.set_cube(dim, next_cube);
            } else {
                for i in 0..next.size[dim.len()] {
                    c(next, cube, &[&dim, &[i][..]].concat())
                }
            }
        }
        let next_size = self.size.iter().map(|s| s + 2).collect::<Vec<_>>();
        let new_data = vec![0; next_size.iter().product::<isize>() as usize];
        let mut next = Cubes::new(new_data, next_size);
        c(&mut next, self, &[]);
        self.size = next.size;
        self.data = next.data;
    }

    fn count_active(&self) -> isize {
        self.data.iter().sum()
    }
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
    run_cycles(data, vec![width, height, 1])
}

fn impl_second_star(contents: &str) -> isize {
    let (data, width, height) = parse_cubes(contents);
    run_cycles(data, vec![width, height, 1, 1])
}

fn run_cycles(data: Vec<isize>, size: Vec<isize>) -> isize {
    let mut cubes = Cubes::new(data, size);
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
