use std::collections::HashSet;

use itertools::Itertools;

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
}

fn get_priority(c: char) -> u32 {
    match c {
        x if x.is_lowercase() => x as u32 - 'a' as u32 + 1,
        x if x.is_uppercase() => x as u32 - 'A' as u32 + 27,
        _ => 1337,
    }
}
pub fn part1(input: &mut [String]) -> u32 {
    input
        .iter()
        .map(|l| {
            (
                l[..l.len() / 2].chars().collect::<HashSet<char>>(),
                l[l.len() / 2..].chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(s1, s2)| get_priority(*s1.intersection(&s2).next().unwrap()))
        .sum::<u32>()
}

pub fn part2(input: &mut [String]) -> u32 {
    let mut iter = input.iter().map(|e| e.chars().collect::<HashSet<char>>());
    let mut sum = 0;
    while let Some(l1) = iter.next() {
        let l2 = iter.next().unwrap();
        let l3 = iter.next().unwrap();
        let s1: HashSet<char> = l1.intersection(&l2).cloned().collect();
        let s2: HashSet<char> = l2.intersection(&l3).cloned().collect();
        let c = s1.intersection(&s2).next().unwrap();
        sum += get_priority(*c);
    }
    sum
}
