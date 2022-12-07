use std::collections::VecDeque;

pub fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let mut v = vec![];
    let mut lines = input.lines();
    loop {
        let mut vv = VecDeque::new();
        let mut line = lines.next().unwrap().chars();
        let mut flag = false;
        loop {
            line.next();
            let ch = line.next();
            match ch {
                Some(x) if x.is_numeric() => {
                    flag = true;
                    break;
                }
                Some(x) => vv.push_back(x),
                None => break,
            }
            line.next();
            line.next();
        }

        if flag {
            break;
        }
        v.push(vv)
    }

    let mut r = vec![];
    for _ in 0..v[0].len() {
        let mut rr = vec![];
        for i in (0..v.len()).rev() {
            let ch = v[i].pop_front().unwrap();
            if ch != ' ' {
                rr.push(ch);
            }
        }
        r.push(rr);
    }
    lines.next();
    let n = lines
        .map(|l| {
            let mut f = l.split(' ');
            f.next();
            let x = u32::from_str_radix(f.next().unwrap(), 10).unwrap();
            f.next();
            let y = u32::from_str_radix(f.next().unwrap(), 10).unwrap();
            f.next();
            let z = u32::from_str_radix(f.next().unwrap(), 10).unwrap();
            (x, y, z)
        })
        .collect();
    (r, n)
}

pub fn part1(input: &mut (Vec<Vec<char>>, Vec<(u32, u32, u32)>)) -> String {
    let (towers, n) = input;
    for (n, from, to) in n.iter() {
        for _ in 0..*n {
            let x = towers[*from as usize - 1].pop().unwrap();
            towers[*to as usize - 1].push(x);
        }
    }
    towers.iter().filter_map(|t| t.last()).collect()
}

pub fn part2(input: &mut (Vec<Vec<char>>, Vec<(u32, u32, u32)>)) -> String {
    let (towers, n) = input;
    let mut stack = vec![];

    for (n, from, to) in n.iter() {
        for _ in 0..*n {
            let x = towers[*from as usize - 1].pop().unwrap();
            stack.push(x);
        }
        while let Some(x) = stack.pop() {
            towers[*to as usize - 1].push(x);
        }
    }
    towers.iter().filter_map(|t| t.last()).collect()
}

fn print_tower(v: &Vec<Vec<char>>) {
    let l = v.iter().map(|vv| v.len()).max().unwrap();
    for i in (0..l).rev() {
        let mut s = String::new();
        for ll in v {
            match ll.get(i) {
                Some(x) => {
                    s.push(*x);
                    s.push(' ');
                }
                None => {
                    s.push(' ');
                    s.push(' ');
                }
            }
        }
        println!("{}", s);
    }
}
