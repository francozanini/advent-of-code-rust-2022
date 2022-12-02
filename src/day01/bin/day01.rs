//! # Advent of Code - Day 1

fn main() {
    let _input = include_str!("../input.txt");

    let mut calories = _input
        .split("\n\n")
        .map(|elf_calories| { elf_calories.lines()
            .map(|x| x.parse::<u32>().unwrap())
            .sum() })
        .collect::<Vec<u32>>();

    calories.sort();

    println!("--- Part One ---");
    println!("Result: {}", calories.last().unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", calories.iter().take(3).sum::<u32>());
}
