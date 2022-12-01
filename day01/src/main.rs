fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut elves = INPUT
        .split("\n\n")
        .map(|e| e.split('\n')
            .map(|l| l.parse::<u64>().unwrap())
            .sum())
        .collect::<Vec<u64>>();

    elves.sort();

    let top = *elves.iter().rev().next().unwrap();
    let top_three: u64 = elves.iter().rev().take(3).sum();

    println!("Top elf: {top}");
    println!("Top three elves: {top_three}");
}
