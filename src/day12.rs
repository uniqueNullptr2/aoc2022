use std::{collections::HashMap, str::FromStr};

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|e| e.chars().collect()).collect()
}

pub fn part1(input: &mut [Vec<char>]) -> u32 {
    rec_path(input, &mut HashMap::new(), 0, 0, 0)
}

pub fn part2(input: &mut [Vec<char>]) -> u32 {
    42
}

fn rec_path(
    map: &[Vec<char>],
    mem: &mut HashMap<(usize, usize), u32>,
    posx: usize,
    posy: usize,
    steps: u32,
) -> u32 {
    let ch = map[posy][posx];
    let mut min = u32::MAX;
    let mut minx = 0;
    let mut miny = 0;
    for (y, x, c) in vec![
        map.get(posy+1)
            .and_then(|e| e.get(posx))
            .map(|e| (posy+1, posx, *e)),
        map.get(posy)
            .and_then(|e| e.get(posx+1))
            .map(|e| (posy, posx+1, *e)),
        map.get(posy.overflowing_sub(1).0)
            .and_then(|e| e.get(posx))
            .map(|e| (posy.overflowing_sub(1).0, posx, *e)),
        map.get(posy)
            .and_then(|e| e.get(posx.overflowing_sub(1).0))
            .map(|e| (posy, posx.overflowing_sub(1).0, *e)),
    ]
    .into_iter()
    .flatten()
    .filter(|(_, _, e)| ch as u8 + 1 <= *e as u8)
    {
        if c == 'E' {
            return 1;
        } else {
            let m = if let Some(s) = mem.get(&(x, y)) {
                *s + steps
            } else {
                rec_path(map, mem, x, y, steps + 1) + steps
            };
            if m < min {
                min = m;
                minx = x;
                miny = y;
            }
        }
    }
    mem.insert((minx, miny), min);
    min
}
