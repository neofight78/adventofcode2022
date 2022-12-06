fn main() {
    const INPUT: &str = include_str!("input.txt");

    let search = |input: &str, size: usize| {
        input
            .chars()
            .collect::<Vec<_>>()
            .windows(size)
            .enumerate()
            .find(|(_, chars)| {
                let mut unique = chars.to_vec();
                unique.sort();
                unique.dedup();
                unique.len() == size
            })
            .unwrap()
            .0
            + size
    };

    println!("Part 1: {}", search(INPUT, 4));
    println!("Part 2: {}", search(INPUT, 14));
}
