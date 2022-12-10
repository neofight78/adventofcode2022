fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut x: isize = 1;
    let mut cycle = 1;
    let mut crt = [[false; 40]; 6];

    let result: isize = INPUT
        .split('\n')
        .flat_map(|l| {
            let mut t = l.split(' ');
            let i = t.next().unwrap();
            let (o, d) = match i {
                "noop" => (None, 1),
                "addx" => {
                    let o = t.next().unwrap().parse::<isize>().unwrap();
                    (Some(o), 2)
                }
                _ => panic!("Unknown instruction!"),
            };

            [1, 0].iter().map(move |&c| (i, o, c)).take(d)
        })
        .map(|(i, o, c)| {
            let x_pos = x % 40;
            let c_pos = ((cycle - 1) / 40, (cycle - 1) % 40);

            if (x_pos - c_pos.1).abs() < 2 {
                crt[c_pos.0 as usize][c_pos.1 as usize] = true;
            }

            if c == 0 && i == "addx" {
                x += o.unwrap();
            }

            cycle += 1;

            (cycle, x)
        })
        .skip(18)
        .step_by(40)
        .map(|(cycle, x)| cycle * x)
        .sum();

    println!("Part 1: {}", result);
    println!("Part 2:");

    for row in crt {
        for (col, pixel) in row.iter().enumerate() {
            if col % 5 == 0 {
                print!("  ");
            }
            print!("{}", if *pixel { '#' } else { ' ' })
        }
        println!();
    }
}
