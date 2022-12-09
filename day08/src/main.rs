use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut map = Vec::new();

    INPUT.split('\n').for_each(|r| {
        map.push(
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        )
    });

    let height = map.len();
    let width = map[0].len();
    let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut visible = HashSet::new();
    let mut max_scenic_score = 0;

    for row in 0..height {
        for col in 0..width {
            let mut viewing_distances = [0; 4];

            'outer: for d in 0..deltas.len() {
                let mut other = (row as isize + deltas[d].0, col as isize + deltas[d].1);

                while other.0 >= 0
                    && other.0 < height as isize
                    && other.1 >= 0
                    && other.1 < width as isize
                {
                    viewing_distances[d] += 1;

                    if map[other.0 as usize][other.1 as usize] >= map[row][col] {
                        continue 'outer;
                    }

                    other = (
                        other.0 as isize + deltas[d].0,
                        other.1 as isize + deltas[d].1,
                    );
                }

                visible.insert((row, col));
            }

            max_scenic_score = max_scenic_score.max(viewing_distances.iter().product());
        }
    }

    println!("Part 1: {}", visible.len());
    println!("Part 2: {max_scenic_score}");
}
