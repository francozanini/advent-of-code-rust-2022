//! # Advent of Code - Day 1 - Part Two

pub fn part2(input: &str) -> usize {
    let lines = input.lines();
    let mut fold = lines.fold(Vec::new(), |mut current, next| {
        let len = current.len();
        let to_add = match str::parse::<usize>(next) {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if next.trim() == "" || len == 0 {
            current.push(if len == 0 { to_add } else { 0 });
        } else {
            current[len - 1] += to_add;
        }

        return current;
    });

    fold.sort();

    return fold
        .into_iter()
        .rev()
        .take(3)
        .reduce(|current, next| current + next)
        .unwrap();
}
