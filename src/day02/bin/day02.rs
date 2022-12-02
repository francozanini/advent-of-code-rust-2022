//! # Advent of Code - Day 2

fn main() {
    let _input = include_str!("../input.txt");

    let answer: u32 = _input.lines()
        .map(|str| return match str.split_at(1) {
            ("A", " X") => 4,
            ("A", " Y") => 8,
            ("A", " Z") => 3,
            ("B", " X") => 1,
            ("B", " Y") => 5,
            ("B", " Z") => 9,
            ("C", " X") => 7,
            ("C", " Y") => 2,
            ("C", " Z") => 6,
            _ => 0
        })
        .sum();

    println!("--- Part One ---");
    println!("Result: {}", answer);

    let answer2: u32 = _input.lines()
        .map(|str| return match str.split_at(1) {
            ("A", " X") => 3,
            ("A", " Y") => 4,
            ("A", " Z") => 8,
            ("B", " X") => 1,
            ("B", " Y") => 5,
            ("B", " Z") => 9,
            ("C", " X") => 2,
            ("C", " Y") => 6,
            ("C", " Z") => 7,
            _ => 0
        })
        .sum();

    println!("--- Part Two ---");
    println!("Result: {}", answer2);
}
