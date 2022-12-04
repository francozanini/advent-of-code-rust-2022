//! # Advent of Code - Day 4

mod part1;
mod part2;

fn range_includes(a: &(u32, u32), b: &(u32, u32)) -> bool {
    return a.0 <= b.0 && a.1 >= b.1;
}

fn range_overlaps(a: &(u32, u32), b: &(u32, u32)) -> bool {
    return (a.0 <= b.0 && a.1 >= b.0) || a.0 <= b.1 && a.1 >= b.1;
}

fn sanitize_digit(digits: &str, prefix_to_remove: &str) -> u32 {
    return digits
        .strip_prefix(prefix_to_remove)
        .unwrap()
        .parse::<u32>()
        .unwrap();
}

fn solve(_input: &str, mut predicate: impl FnMut(&((u32, u32), (u32, u32))) -> bool) -> usize {
    _input
        .lines()
        .map(|line| {
            let (first_pair, second_pair) = line.split_at(line.find(",").unwrap());
            let first_pair = first_pair.split_at(first_pair.find("-").unwrap());
            let second_pair = second_pair.split_at(second_pair.find("-").unwrap());
            return (
                (
                    first_pair.0.parse::<u32>().unwrap(),
                    sanitize_digit(first_pair.1, "-"),
                ),
                (
                    sanitize_digit(second_pair.0, ","),
                    sanitize_digit(second_pair.1, "-"),
                ),
            );
        })
        .filter(|pairs| predicate(pairs))
        .count()
}

fn main() {
    let _input = include_str!("../input.txt");

    let part1 = solve(_input, |(range_a, range_b)| {
        range_includes(range_a, range_b) || range_includes(range_b, range_a)
    });

    println!("--- Part One ---");
    println!("Result: {}", part1);

    let part2 = solve(_input, |(range_a, range_b)| {
        range_overlaps(range_a, range_b) || range_overlaps(range_b, range_a)
    });
    println!("--- Part Two ---");
    println!("Result: {}", part2);
}
