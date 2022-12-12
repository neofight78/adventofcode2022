use std::cmp::Reverse;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone)]
enum Operand {
    Number(u128),
    Old,
}

#[derive(Clone)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test_denominator: u128,
    true_monkey: usize,
    false_monkey: usize,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        fn get_number_at_eol<'a, T>(lines: &mut impl Iterator<Item = &'a str>) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap()
        }

        let mut lines = input.split('\n').map(|l| l.split_once(':').unwrap().1);
        lines.next();

        let items = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|i| i.trim().parse().unwrap())
            .collect::<VecDeque<_>>();

        let mut operation = lines.next().unwrap().split(' ').skip(4);
        let operator = match operation.next().unwrap() {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator!"),
        };
        let operand = match operation.next().unwrap() {
            "old" => Operand::Old,
            n => Operand::Number(n.parse().unwrap()),
        };

        Self {
            items,
            operation: Operation { operand, operator },
            test_denominator: get_number_at_eol(&mut lines),
            true_monkey: get_number_at_eol(&mut lines),
            false_monkey: get_number_at_eol(&mut lines),
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut monkeys = INPUT.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    let mut monkeys2 = monkeys.clone();
    let product: u128 = monkeys.iter().map(|m| m.test_denominator).product();

    let part1 = monkey_business(&mut monkeys, 20, |i| i / 3);
    let part2 = monkey_business(&mut monkeys2, 10000, |i| i % product);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn monkey_business<F: Fn(u128) -> u128>(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    limit: F,
) -> u128 {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                inspections[i] += 1;

                let operand = match monkeys[i].operation.operand {
                    Operand::Number(n) => n,
                    Operand::Old => item,
                };
                item = match monkeys[i].operation.operator {
                    Operator::Add => item + operand,
                    Operator::Multiply => item * operand,
                };

                item = limit(item);

                let pass_to = if item % monkeys[i].test_denominator == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[pass_to].items.push_back(item);
            }
        }
    }

    inspections.select_nth_unstable_by_key(1, |x| Reverse(*x));
    inspections[0] * inspections[1]
}
