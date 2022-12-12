use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = INPUT
        .split('\n')
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = (row, col);
                        0
                    }
                    'E' => {
                        end = (row, col);
                        25
                    }
                    _ => c as u8 - b'a',
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    let mut distances = vec![vec![usize::MAX; width]; height];
    let mut shortest_distance = usize::MAX;
    let mut queue = BinaryHeap::new();

    distances[end.0][end.1] = 0;
    queue.push((Reverse(0), end));

    while let Some((Reverse(distance), position)) = queue.pop() {
        if distance > distances[position.0][position.1] {
            continue;
        }

        if map[position.0][position.1] == 0 && distance < shortest_distance {
            shortest_distance = distance;
        }

        for delta in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_position = (position.0 as isize + delta.0, position.1 as isize + delta.1);

            if new_position.0 < 0
                || new_position.0 >= height as isize
                || new_position.1 < 0
                || new_position.1 >= width as isize
            {
                continue;
            }

            let new_distance = distance + 1;
            let new_position = (new_position.0 as usize, new_position.1 as usize);

            if map[new_position.0][new_position.1] + 1 < map[position.0][position.1] {
                continue;
            }

            if new_distance < distances[new_position.0][new_position.1] {
                distances[new_position.0][new_position.1] = new_distance;
                queue.push((Reverse(new_distance), new_position));
            }
        }
    }

    println!("Part 1: {}", distances[start.0][start.1]);
    println!("Part 2: {}", shortest_distance);
}
