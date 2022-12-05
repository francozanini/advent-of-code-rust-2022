//! # Advent of Code - Day 5

use std::collections::VecDeque;

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
    let mut state = parse_crate_state(_input).clone();
    let max = state.iter().map(|row| row.len()).max().unwrap() + 1;
    let instructions = _input
        .lines()
        .enumerate()
        .filter(|(i, _crates)| i.clone() > (max))
        .map(|(_, instruction)| instruction);

    for instruction in instructions {
        let parsed_instruction: Vec<usize> = instruction
            .split_whitespace()
            .filter(|word| !["move", "from", "to"].contains(word))
            .map(|n| n.parse().unwrap())
            .collect();

        println!("before {:?}", state);

        let amount_to_move = parsed_instruction[0];
        let mut to_move = vec![];

        println!("parsed {:?}", parsed_instruction);

        println!(
            "move {} from {} to {}",
            amount_to_move,
            parsed_instruction[1] - 1,
            parsed_instruction[2] - 1
        );

        for _ in 0..amount_to_move {
            let maybe_removed = state
                .get_mut(parsed_instruction[1] - 1)
                .unwrap()
                .pop_front();

            if maybe_removed.is_some() {
                to_move.push(maybe_removed.unwrap())
            }
        }

        for item in to_move {
            state
                .get_mut(parsed_instruction[2] - 1)
                .unwrap()
                .push_front(item)
        }

        println!("after {:?}", state);
    }

    let default = "".to_string();

    println!(
        "{:?}",
        state
            .iter()
            .map(|deque| deque.front().unwrap_or(&default))
            .collect::<Vec<&String>>()
    );

    println!("--- Part One ---");
    println!("Result: {}", part1::part1());

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2());
}

fn parse_crate_state(_input: &str) -> Vec<VecDeque<String>> {
    let mut matrix: Vec<Vec<&str>> = vec![];
    for line in _input.lines().take_while(|line| !line.contains("1")) {
        matrix.push(line.split("").collect());
    }
    println!("{:?}", matrix);

    let transposed = transpose(matrix);
    println!("{:?}", transposed);
    return transposed
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|symbol| {
                    !symbol.contains("[") && !symbol.contains("]") && !symbol.trim().is_empty()
                })
                .map(|symbol| symbol.to_string())
                .collect::<VecDeque<String>>()
        })
        .filter(|row| !row.is_empty())
        .collect();
}
