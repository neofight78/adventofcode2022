use std::collections::HashSet;

type Input = Vec<((isize, isize), (isize, isize))>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    const PART1_ROW: isize = 2000000;
    const PART1_BOUNDS: (isize, isize) = (isize::MIN, isize::MAX);
    const PART2_BOUNDS: (isize, isize) = (0, 4000000);

    let sensors_and_beacons = parse_input(INPUT);

    // Part 1
    let coverage = coverage_for_row(&sensors_and_beacons, PART1_BOUNDS, PART1_ROW);
    let excluded = count_coverage_in_row(&coverage);
    let beacons = count_beacons_in_row(&sensors_and_beacons, PART1_ROW);
    println!("Part 1: {}", excluded - beacons);

    // Part 2
    let beacon_location = find_beacon(&sensors_and_beacons, PART2_BOUNDS);
    let signal_strength = beacon_location.0 * 4000000 + beacon_location.1;
    println!("Part 2: {signal_strength}");
}

fn parse_input(input: &str) -> Input {
    input
        .split('\n')
        .map(|l| {
            let mut coords = l.split(' ').filter_map(|c| {
                c.chars()
                    .filter(|c| c.is_ascii_digit() || *c == '-')
                    .collect::<String>()
                    .parse::<isize>()
                    .ok()
            });

            (
                (coords.next().unwrap(), coords.next().unwrap()),
                (coords.next().unwrap(), coords.next().unwrap()),
            )
        })
        .collect::<Vec<_>>()
}

fn coverage_for_row(
    sensors_and_beacons: &Input,
    x_bounds: (isize, isize),
    row: isize,
) -> Vec<(isize, isize)> {
    let mut coverage = Vec::new();
    let mut beacons = HashSet::new();

    for (sensor, beacon) in sensors_and_beacons {
        beacons.insert(beacon);

        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        if row < sensor.1 - distance || row > sensor.1 + distance {
            continue;
        }

        let y_diff = (sensor.1 - row).abs();
        let mut x_start = sensor.0 - (distance - y_diff);
        let mut x_end = sensor.0 + (distance - y_diff);

        x_start = x_start.max(x_bounds.0);
        x_end = x_end.min(x_bounds.1);

        coverage.push((x_start, x_end));
    }

    let mut merged = Vec::<(isize, isize)>::new();

    'ranges: while let Some(range) = coverage.pop() {
        for i in 0..merged.len() {
            if range.0 >= merged[i].0 && range.0 <= merged[i].1
                || range.1 >= merged[i].0 && range.1 <= merged[i].1
                || range.0 <= merged[i].0 && range.1 >= merged[i].1
            {
                let new_range = (merged[i].0.min(range.0), merged[i].1.max(range.1));

                coverage.push(new_range);
                merged.swap_remove(i);
                continue 'ranges;
            }
        }

        merged.push(range);
    }

    merged
}

fn count_coverage_in_row(coverage: &[(isize, isize)]) -> isize {
    coverage.iter().map(|r| (r.1 - r.0) + 1).sum()
}

fn count_beacons_in_row(sensors_and_beacons: &Input, row: isize) -> isize {
    let beacons = sensors_and_beacons
        .iter()
        .map(|(_, b)| b)
        .filter(|b| b.1 == row)
        .collect::<HashSet<_>>();

    beacons.len() as isize
}

fn find_beacon(sensors_and_beacons: &Input, bounds: (isize, isize)) -> (isize, isize) {
    for row in bounds.0..=bounds.1 {
        let coverage = coverage_for_row(sensors_and_beacons, bounds, row);
        let excluded = count_coverage_in_row(&coverage);

        if excluded == bounds.1 - bounds.0 {
            let x = if coverage[0].0 == 0 {
                coverage[0].1 + 1
            } else {
                coverage[0].0 - 1
            };

            return (x, row);
        }
    }

    panic!("Failed to find the beacon!");
}
