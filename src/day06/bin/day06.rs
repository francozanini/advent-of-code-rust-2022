//! # Advent of Code - Day 6

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

struct SignalLocker {
    data_stream: Vec<char>,
}

impl FromStr for SignalLocker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(SignalLocker::new(s.chars().collect()));
    }
}

impl SignalLocker {
    fn new(data_stream: Vec<char>) -> SignalLocker {
        SignalLocker { data_stream }
    }

    fn find_package_start(&self, signal_len: usize) -> Result<usize, &str> {
        let mut last_three: VecDeque<&char> = VecDeque::new();
        for (index, signal) in self.data_stream.iter().enumerate() {
            last_three.push_back(signal);

            if last_three.iter().collect::<HashSet<&&char>>().len() == signal_len {
                return Ok(index + 1);
            }

            if last_three.len() >= signal_len {
                last_three.pop_front();
            }
        }

        return Err("bad input");
    }
}

fn main() {
    let _input = include_str!("../input.txt");
    let signal_locker = _input.parse::<SignalLocker>().expect("input is valid");

    println!("--- Part One ---");
    println!(
        "{}",
        signal_locker
            .find_package_start(4)
            .expect("obtained result")
    );

    println!("--- Part One ---");
    println!(
        "{}",
        signal_locker
            .find_package_start(14)
            .expect("obtained result")
    );
}
