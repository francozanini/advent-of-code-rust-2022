//! # Advent of Code - Day 5

use std::fmt::Debug;
use std::ops::Index;

mod part1;
mod part2;

fn transpose(v: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    return (0..v.iter().map(|row| row.len()).max().unwrap())
        .map(|i| {
            v.iter()
                .map(|inner| inner.get(i).unwrap_or(&"").clone())
                .collect::<Vec<&str>>()
        })
        .collect();
}

fn main() {
    let _input = include_str!("../input.txt");
    let mut matrix: Vec<Vec<&str>> = vec![];
    for line in _input.lines().take_while(|line| !line.contains("1")) {
        matrix.push(line.split("").collect());
    }
    println!(
        "{:?}",
        transpose(matrix)
            .iter_mut()
            .map(|row| row
                .iter()
                .filter(|symbol| !symbol.contains("[")
                    && !symbol.contains("]")
                    && !symbol.trim().is_empty())
                .cloned()
                .collect::<Vec<&str>>())
            .filter(|row| !row.is_empty())
            .collect::<Vec<Vec<&str>>>()
    );

    println!("--- Part One ---");
    println!("Result: {}", part1::part1());

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2());
}
