use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

pub fn parse(input: &str) -> String {
    input.to_owned()
}

pub fn part1(input: &str) -> u32 {
    let mut mem = VecDeque::new();
    for (i, ch) in input.chars().enumerate() {
        mem.push_back(ch);
        if i > 3 {
            mem.pop_front();
            if mem.iter().unique().count() == 4 {
                return i as u32 + 1;
            }
        }
    }
    1337
}

pub fn part2(input: &str) -> u32 {
    let mut mem = VecDeque::new();
    for (i, ch) in input.chars().enumerate() {
        mem.push_back(ch);
        if i > 13 {
            mem.pop_front();
            if mem.iter().unique().count() == 14 {
                return i as u32 + 1;
            }
        }
    }
    1337
}
