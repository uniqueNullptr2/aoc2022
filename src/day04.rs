use std::str::FromStr;

pub fn parse(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .map(|l| {
            let mut p = l.split(',');
            let mut e = p.next().unwrap().split('-');
            let (f1, f2) = (
                u32::from_str(e.next().unwrap()).unwrap(),
                u32::from_str(e.next().unwrap()).unwrap(),
            );
            e = p.next().unwrap().split('-');
            (
                f1,
                f2,
                u32::from_str(e.next().unwrap()).unwrap(),
                u32::from_str(e.next().unwrap()).unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &mut [(u32, u32, u32, u32)]) -> usize {
    input
        .iter()
        .filter(|(e1, e2, f1, f2)| (e1 <= f1 && e2 >= f2) || (f1 <= e1 && f2 >= e2))
        .count()
}

pub fn part2(input: &mut [(u32, u32, u32, u32)]) -> usize {
    input
        .iter()
        .filter(|(e1, e2, f1, f2)| {
            (e1 <= f1 && e2 >= f1)
                || (e1 <= f2 && e2 >= f2)
                || (f1 <= e1 && f2 >= e1)
                || (f1 <= e2 && f2 >= e2)
        })
        .count()
}
