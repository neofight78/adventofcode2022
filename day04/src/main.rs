fn main() {
    const INPUT: &str = include_str!("input.txt");

    let totals = INPUT
        .split('\n')
        .map(|p| {
            let mut e = p.split(',').map(|r| {
                let mut b = r.split('-').map(|s| s.parse::<u64>().unwrap());
                (b.next().unwrap(), b.next().unwrap())
            });

            let a = e.next().unwrap();
            let b = e.next().unwrap();

            let a0 = a.0 >= b.0 && a.0 <= b.1;
            let a1 = a.1 >= b.0 && a.1 <= b.1;
            let b0 = b.0 >= a.0 && b.0 <= a.1;
            let b1 = b.1 >= a.0 && b.1 <= a.1;

            (
                u64::from((a0 && a1) || (b0 && b1)),
                u64::from(a0 || a1 || b0 || b1),
            )
        })
        .fold((0, 0), |a, x| (a.0 + x.0, a.1 + x.1));

    println!("Part 1: {}", totals.0);
    println!("Part 2: {}", totals.1);
}
