fn main() {
    const INPUT: &str = include_str!("input.txt");

    let part1_algo = |r: &str| -> u64 {
        match r.split_once(' ').unwrap() {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            (o, y) => panic!("Unexpected character ('{}', '{}')", o, y)
        }
    };

    let part2_algo = |r: &str| -> u64 {
        match r.split_once(' ').unwrap() {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            (o, y) => panic!("Unexpected character ('{}', '{}')", o, y)
        }
    };

    let part1_total: u64 = INPUT.split('\n').map(part1_algo).sum();
    let part2_total: u64 = INPUT.split('\n').map(part2_algo).sum();

    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}");
}
