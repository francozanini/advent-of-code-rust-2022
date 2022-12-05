//! # Advent of Code - Day 5

use std::collections::VecDeque;
use std::iter::{Enumerate, Filter, Map};

fn transpose(v: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    return (0..v.iter().map(|row| row.len()).max().unwrap())
        .map(|i| {
            v.iter()
                .map(|inner| inner.get(i).unwrap_or(&"").clone())
                .collect::<Vec<&str>>()
        })
        .collect();
}

fn parse_crate_state(_input: &str) -> Vec<VecDeque<String>> {
    let mut matrix: Vec<Vec<&str>> = vec![];
    for line in _input.lines().take_while(|line| !line.contains("1")) {
        matrix.push(line.split("").collect());
    }

    return transpose(matrix)
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

fn print_result(final_state: Vec<VecDeque<String>>) -> Vec<String> {
    let default = "".to_string();

    final_state
        .iter()
        .map(|deque| deque.front().unwrap_or(&default).clone())
        .collect::<Vec<String>>()
}

fn run_instructions(
    initial_state: Vec<VecDeque<String>>,
    instructions: Vec<&str>,
) -> Vec<VecDeque<String>> {
    let mut state = initial_state.clone();

    for instruction in instructions {
        let parsed_instruction: Vec<usize> = instruction
            .split_whitespace()
            .filter(|word| !["move", "from", "to"].contains(word))
            .map(|n| n.parse().unwrap())
            .collect();

        let amount_to_move = parsed_instruction[0];
        let mut to_move = vec![];

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
    }

    return state;
}

fn find_instructions(_input: &str, max: usize) -> Vec<&str> {
    _input
        .lines()
        .enumerate()
        .filter(|(i, _crates)| i.clone() > (max))
        .map(|(_, instruction)| instruction)
        .collect()
}

fn main() {
    let _input = include_str!("../input.txt");
    let state = parse_crate_state(_input).clone();
    let max = state.iter().map(|row| row.len()).max().unwrap() + 1;
    let instructions = find_instructions(_input, max);

    let result_one = run_instructions(state, instructions);

    println!("--- Part One ---");
    println!("Result: {:?}", print_result(result_one));

    println!("--- Part Two ---");
    println!("Result: {}", "");
}
