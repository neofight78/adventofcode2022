use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut rope_1 = [(0, 0); 2];
    let mut rope_2 = [(0, 0); 10];

    let mut visited_by_tail_1 = HashSet::new();
    let mut visited_by_tail_2 = HashSet::new();

    INPUT.split('\n').for_each(|l| {
        let (direction, distance) = l.split_once(' ').unwrap();
        let distance = distance.parse::<usize>().unwrap();

        let delta = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("parse error"),
        };

        for _ in 0..distance {
            move_rope(&mut rope_1, delta, &mut visited_by_tail_1);
            move_rope(&mut rope_2, delta, &mut visited_by_tail_2);
        }
    });

    println!("Part 1: {}", visited_by_tail_1.len());
    println!("Part 2: {}", visited_by_tail_2.len());
}

fn move_rope<const L: usize>(
    rope: &mut [(isize, isize); L],
    delta: (isize, isize),
    visited: &mut HashSet<(isize, isize)>,
) {
    rope[0] = (rope[0].0 + delta.0, rope[0].1 + delta.1);

    for i in 1..L {
        let diff = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);

        if diff.0.abs() == 2 || diff.1.abs() == 2 {
            rope[i] = (
                rope[i].0 + diff.0.max(-1).min(1),
                rope[i].1 + diff.1.max(-1).min(1),
            );
        }
    }

    visited.insert(rope[L - 1]);
}
