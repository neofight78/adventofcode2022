use std::ops::{Index, IndexMut};

struct Cave {
    min_x: usize,
    tiles: Vec<Vec<bool>>,
}

impl Cave {
    fn new(min_x: isize, max_x: isize, max_y: isize) -> Self {
        Self {
            min_x: (min_x - max_y) as usize,
            tiles: vec![vec![false; (max_y * 2 + max_x - min_x) as usize]; max_y as usize + 2],
        }
    }
}

impl Index<(isize, isize)> for Cave {
    type Output = bool;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.tiles[y as usize][x as usize - self.min_x]
    }
}

impl IndexMut<(isize, isize)> for Cave {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        &mut self.tiles[y as usize][x as usize - self.min_x]
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let paths = INPUT
        .split('\n')
        .map(|l| {
            l.split(" -> ")
                .flat_map(|p| {
                    p.split_once(',')
                        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let min_x = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.0))
        .min()
        .unwrap();
    let max_x = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.0))
        .max()
        .unwrap();
    let max_y = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.1))
        .max()
        .unwrap();

    let mut cave = Cave::new(min_x, max_x, max_y);

    for path in paths {
        let mut pos = path[0];

        for coord in path {
            let delta = (
                (coord.0 - pos.0).clamp(-1, 1),
                (coord.1 - pos.1).clamp(-1, 1),
            );

            while pos != coord {
                cave[pos] = true;
                pos = (pos.0 + delta.0, pos.1 + delta.1);
            }
            cave[pos] = true;
        }
    }

    let mut units = 0;
    let mut into_abyss = 0;

    'sand: loop {
        let mut pos = (500, 0);
        units += 1;

        loop {
            if pos.1 > max_y && into_abyss == 0 {
                into_abyss = units;
            }

            if pos.1 == max_y + 1 {
                cave[pos] = true;
                continue 'sand;
            }

            pos = if !cave[(pos.0, pos.1 + 1)] {
                (pos.0, pos.1 + 1)
            } else if !cave[(pos.0 - 1, pos.1 + 1)] {
                (pos.0 - 1, pos.1 + 1)
            } else if !cave[(pos.0 + 1, pos.1 + 1)] {
                (pos.0 + 1, pos.1 + 1)
            } else {
                cave[pos] = true;

                if pos == (500, 0) {
                    break 'sand;
                }
                continue 'sand;
            }
        }
    }

    println!("Part 1: {into_abyss}");
    println!("Part 2: {units}");
}
