//! # Advent of Code - Day 1 - Part One

pub fn part1(input: &str) -> usize {
    let lines = input.lines();
    let mut max = 0;
    let mut current = 0;
    lines.for_each(|next| {
        if next.trim() == "" {
            max = if current > max { current } else { max };
            current = 0;
        }
        current += match str::parse::<usize>(next) {
            Ok(v) => v,
            Err(e) => 0,
        };
    });

    return max;
}
