fn main() {
    const INPUT: &str = include_str!("input.txt");

    let score = |c: char| if c > 'Z' { c as u8 - b'`' } else { c as u8 + 26 - b'@' } as u64;

    let part1_total: u64 = INPUT
        .split('\n')
        .map(|r| r.split_at(r.len() / 2))
        .map(|(a, b)| a.chars().find(|&ac| b.chars().any(|bc| ac == bc)).unwrap())
        .map(score)
        .sum();

    let part2_total: u64 = INPUT
        .split('\n')
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|r| {
            r[0].chars()
                .find(|&r0| r[1].chars().any(|r1| r0 == r1) && r[2].chars().any(|r2| r0 == r2))
                .unwrap()
        })
        .map(score)
        .sum();

    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}");
}
