use std::fs;
use std::collections::{ HashMap, HashSet };
use text_io::scan;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day20.txt")
        .expect("Something went wrong reading the file");

    let res = impl_first_star(&contents);

    println!("day 20.1 - Product of corner tiles IDs: {}", res);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day20.txt")
        .expect("Something went wrong reading the file");

    let res = impl_second_star(&contents);

    println!("day 20.2 - number of # not part of a sea monster: {}", res);
}

fn impl_first_star(contents: &str) -> usize {
    let tiles = parse_tiles(contents);
    let borders = extract_borders(&tiles);
    let neighbours = find_neighbours(&borders);
    find_corners(&neighbours)
        .iter()
        .product()
}

fn impl_second_star(contents: &str) -> usize {
    let tiles = parse_tiles(contents);
    let borders = extract_borders(&tiles);
    let neighbours = find_neighbours(&borders);
    let corners = find_corners(&neighbours);
    let tilemap = create_tilemap(&neighbours, *corners.first().unwrap());
    let mut grid = create_map(&tilemap, &tiles, &borders);
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
        ].iter()
            .map(|line|line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
    find_monsters(&mut grid, &monster);
    grid.iter()
        .map(|row| row.iter().filter(|&c| c == &'#').count())
        .sum()
}

type Pos = (i32, i32);
type Grid = Vec<Vec<char>>;
type Borders = Vec<Vec<char>>;
type Neighbours = HashSet<usize>;
type Tilemap = HashMap<Pos, usize>;

const N: usize = 0;
const S: usize = 1;
const W: usize = 2;
const E: usize = 3;

const DN: Pos = (-1, 0);
const DS: Pos = (1, 0);
const DW: Pos = (0, -1);
const DE: Pos = (0, 1);

fn parse_tiles(contents: &str) -> HashMap<usize, Grid> {
    contents.split("\n\n")
        .filter_map(|tile| {
            if tile.is_empty() {
                None
            } else {
                let mut lines = tile.lines();
                let id: usize;
                scan!(lines.next().unwrap().bytes() => "Tile {}:", id);
                let grid = lines
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                Some((id, grid))
            }
        })
        .collect::<HashMap<_,_>>()
}

fn extract_borders(tiles: &HashMap<usize, Grid>) -> HashMap<usize, Borders> {
    tiles.iter()
        .map(|(&id, grid)| {
            let mut borders = vec![
                grid.first().unwrap().clone(),
                grid.last().unwrap().clone(),
                grid.iter().map(|g| *g.first().unwrap()).collect(),
                grid.iter().map(|g| *g.last().unwrap()).collect()
            ];
            let borders_reversed = borders.iter()
                .map(|b| {
                    let mut rev = b.clone();
                    rev.reverse();
                    rev
                })
                .collect::<Vec<_>>();
            borders.extend(borders_reversed);
            (id, borders)
        })
        .collect::<HashMap<_,_>>()
}

fn find_neighbours(borders: &HashMap<usize, Borders>) -> HashMap<usize, Neighbours> {
    borders.iter()
        .map(|(&id, list)| {
            let set = list.iter().collect::<HashSet<_>>();
            (id,
             borders.iter()
                 .filter_map(|(&i, b)|
                     if i == id || set.intersection(&b.iter().collect()).collect::<HashSet<_>>().is_empty() {
                         None
                     } else {
                         Some(i)
                     }
                 )
                 .collect())
        })
        .collect()
}

fn find_corners(neighbours: &HashMap<usize, Neighbours>) -> Vec<usize> {
    neighbours.iter()
        .filter_map(|(&id, set)|
            if set.len() == 2 { Some(id) } else { None }
        )
        .collect()
}

fn create_tilemap(neighbours: &HashMap<usize, Neighbours>, corner: usize) -> Tilemap {
    let side = (neighbours.len() as f32).sqrt() as i32;
    let mut n = neighbours.clone();
    n.remove(&corner);
    let mut tilemap = HashMap::new();
    tilemap.insert((0, 0), corner);

    let mut c = 1;
    let mut r = 0;
    while !n.is_empty() {
        let neighbours = [DN, DS, DW, DE]
            .iter()
            .filter_map(|(dr, dc)| tilemap.get(&(r + dr, c + dc)))
            .cloned()
            .collect::<HashSet<_>>();
        let next = n.iter()
            .filter_map(|(&id, set)|
                if set.is_superset(&neighbours) &&
                    set.len() <= neighbours.len() + 2 { Some(id) } else { None }
            )
            .collect::<Vec<_>>();
        let id = next.first().unwrap();
        tilemap.insert((r, c), *id);
        n.remove(id);
        c = (c + 1) % side;
        if c == 0 { r += 1; }
    }

    tilemap
}

fn create_map(tilemap: &Tilemap, tiles: &HashMap<usize, Grid>, borders: &HashMap<usize, Borders>) -> Grid {
    let tiles_with_edges = tilemap.iter()
        .map(|((r, c), id)| {
            let edges = [DN, DS, DW, DE].iter()
                .map(|(dr, dc)|
                    tilemap.get(&(r + dr, c + dc))
                        .map(|next|
                            borders.get(id).unwrap()
                                .iter()
                                .position(|border|
                                    borders.get(next).unwrap().contains(&border)
                                )
                                .unwrap()
                        )
                )
                .collect::<Vec<_>>();
            (*id, edges)
        })
        .collect::<HashMap<_, _>>();

    let adjusted_tiles = adjust_tiles(&tiles_with_edges, &tiles);
    let mut grid = Vec::new();
    let tile_len = tiles.values().next().unwrap().len();
    let side = (tilemap.len() as f32).sqrt() as i32;

    for r in 0..side {
        for i in 1..tile_len-1 {
            let mut row = String::new();
            for c in 0..side {
                let g = adjusted_tiles.get(tilemap.get(&(r, c)).unwrap()).unwrap();
                row = format!("{}{}", row, g[i].iter().take(tile_len - 1).skip(1).collect::<String>())
            }
            grid.push(row.chars().collect::<Vec<_>>());
        }
    }

    grid
}

fn adjust_tiles(tiles_with_edges: &HashMap<usize, Vec<Option<usize>>>, tiles: &HashMap<usize, Grid>) -> HashMap<usize, Grid> {
    tiles_with_edges.iter()
        .map(|(id, edges)| {
            let mut grid = tiles.get(id).unwrap().clone();
            if edges[N] == Some(W) ||
                edges[N] == Some(E) ||
                edges[S] == Some(W) ||
                edges[S] == Some(E) {
                rotate_cw(&mut grid);
            }
            if edges[N] == Some(S) ||
                edges[S] == Some(N) ||
                edges[N] == Some(E) ||
                edges[S] == Some(W) {
                mirror_vert(&mut grid);
            }
            if edges[W] == Some(E) ||
                edges[E] == Some(W) ||
                edges[W] == Some(N) ||
                edges[E] == Some(S) {
                mirror_horiz(&mut grid);
            }
            (*id, grid)
        })
        .collect::<HashMap<_,_>>()
}

fn find_monsters(grid: &mut Grid, monster: &[Vec<char>]) {
    let monster_coord = monster.iter()
        .enumerate()
        .flat_map(|(r, row)|
            row.iter()
                .enumerate()
                .filter_map(|(c, t)| if *t == '#' { Some((r, c)) } else { None })
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    map_monster(grid, &monster_coord);

    mirror_horiz(grid);
    map_monster(grid, &monster_coord);

    mirror_vert(grid);
    map_monster(grid, &monster_coord);

    mirror_horiz(grid);
    map_monster(grid, &monster_coord);

    rotate_cw(grid);
    map_monster(grid, &monster_coord);

    mirror_horiz(grid);
    map_monster(grid, &monster_coord);

    mirror_vert(grid);
    map_monster(grid, &monster_coord);

    mirror_horiz(grid);
    map_monster(grid, &monster_coord);
}

fn map_monster(grid: &mut Grid, monster_coord: &[(usize, usize)]) {
    let side = grid.len();
    for r in 0..side {
        for c in 0..side {
            if monster_coord.iter().all(|(dr, dc)|
                if let Some(row) = grid.get(r + dr) {
                    row.get(c + dc) == Some(&'#')
                } else {
                    false
                }
            ) {
                monster_coord.iter()
                    .for_each(|(dr, dc)| grid[r + dr][c + dc] = 'O')
            }
        }
    }
}

fn mirror_vert(grid: &mut Grid) {
    grid.reverse();
}

fn mirror_horiz(grid: &mut Grid) {
    grid.iter_mut().for_each(|row| row.reverse());
}

fn rotate_cw(grid: &mut Grid) {
    let side = grid.len();
    let mut new_grid = vec![vec![Default::default(); side]; side];
    for (r, row) in grid.iter().enumerate() {
        for (c, t) in row.iter().enumerate() {
            new_grid[c][side - r - 1] = *t;
        }
    }
    *grid = new_grid;
}

#[test]
fn test0_first_star() {
    let contents = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    assert_eq!(impl_first_star(contents), 20899048083289);
}

#[test]
fn test0_second_star() {
    let contents = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    assert_eq!(impl_second_star(contents), 273);
}
