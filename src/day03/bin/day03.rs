//! # Advent of Code - Day 3
use std::collections::HashSet;

fn intersection(first: &str, second: &str) -> String {
    let set_a: HashSet<String> = HashSet::from_iter(first.split("").map(|s| s.to_string()));
    let set_b: HashSet<String> = HashSet::from_iter(second.split("").map(|s| s.to_string()));
    return set_a
        .intersection(&set_b)
        .filter(|str| !str.is_empty())
        .cloned()
        .collect::<String>();
}

fn item_value(item: char) -> usize {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    return alphabet.chars().position(|char| char == item).unwrap() + 1;
}

fn part1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first_half, second_half)| {
            intersection(first_half, second_half)
                .chars()
                .last()
                .unwrap()
        })
        .map(|item| item_value(item))
        .sum();
}

fn part2(input: &str) -> usize {
    return input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| intersection(intersection(chunk[0], chunk[1]).as_str(), chunk[2]))
        .map(|item| item_value(item.chars().last().unwrap()))
        .sum();
}

fn main() {
    let _input = include_str!("../input.txt");

    println!("--- Part One ---");
    println!("Result: {:?}", part1(_input));

    println!("--- Part Two ---");
    println!("Result: {}", part2(_input));
}
