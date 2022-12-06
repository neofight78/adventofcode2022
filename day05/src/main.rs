use std::collections::VecDeque;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let (drawing, instructions) = INPUT.split_once("\n\n").unwrap();
    let (drawing, columns) = drawing.split_once("\n 1").unwrap();
    let columns = columns.split_whitespace().count() + 1;

    let mut stacks = vec![VecDeque::<char>::new(); columns];

    drawing.split('\n').for_each(|l| {
        l.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .enumerate()
            .for_each(|(pos, chars)| {
                if chars[1] != ' ' {
                    stacks[pos].push_front(chars[1])
                }
            })
    });

    let mut stacks2 = stacks.clone();

    instructions.split('\n').for_each(|l| {
        let mut numbers = l.split_whitespace().filter_map(|t| t.parse::<usize>().ok());
        let qty = numbers.next().unwrap();
        let from = numbers.next().unwrap() - 1;
        let to = numbers.next().unwrap() - 1;

        for _ in 0..qty {
            let c = stacks[from].pop_back().unwrap();
            stacks[to].push_back(c);
        }

        let at = stacks2[from].len() - qty;
        let mut group = stacks2[from].split_off(at);
        stacks2[to].append(&mut group);
    });

    let part1: String = stacks.iter().map(|s| s[s.len() - 1]).collect();
    let part2: String = stacks2.iter().map(|s| s[s.len() - 1]).collect();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
