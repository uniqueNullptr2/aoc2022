use std::str::FromStr;

pub fn parse(input: &str) -> Vec<u32> {
    input
        .split("\r\n\r\n")
        .map(|e| e.lines().map(|f| u32::from_str(f).unwrap()).sum::<u32>())
        .collect::<Vec<u32>>()
}

pub fn part1(input: &mut [u32]) -> u32 {
    *input.iter().max().unwrap()
}

pub fn part2(input: &mut [u32]) -> u32 {
    input.sort();
    input.iter().rev().take(3).sum()
}
