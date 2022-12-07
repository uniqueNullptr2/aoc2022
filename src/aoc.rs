use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::{day01, day02, day03, day04, day05, day06, day07};

fn run(day: usize) -> Option<AocResult> {
    std::fs::read_to_string(format!("inputs/day{:02}.txt", day))
        .ok()
        .map(|s| match day {
            1 => Some(run_day_mut(
                "AOC 2021 Day01",
                &s,
                day01::parse,
                |input| day01::part1(input),
                |input| day01::part2(input),
            )),
            2 => Some(run_day_mut(
                "AOC 2021 Day02",
                &s,
                day02::parse,
                |input| day02::part1(input),
                |input| day02::part2(input),
            )),
            3 => Some(run_day_mut(
                "AOC 2021 Day03",
                &s,
                day03::parse,
                |input| day03::part1(input),
                |input| day03::part2(input),
            )),
            4 => Some(run_day_mut(
                "AOC 2021 Day04",
                &s,
                day04::parse,
                |input| day04::part1(input),
                |input| day04::part2(input),
            )),
            5 => Some(run_day_mut(
                "AOC 2021 Day05",
                &s,
                day05::parse,
                |input| day05::part1(input),
                |input| day05::part2(input),
            )),
            6 => Some(run_day(
                "AOC 2021 Day06",
                &s,
                day06::parse,
                |input| day06::part1(input),
                |input| day06::part2(input),
            )),
            7 => Some(run_day_mut(
                "AOC 2021 Day07",
                &s,
                day07::parse,
                |input| day07::part1(input),
                |input| day07::part2(input),
            )),
            _ => None,
        })
        .flatten()
}

pub struct AocResult {
    name: String,
    gen_time: Duration,
    p1_time: Duration,
    p2_time: Duration,
    e1: String,
    e2: String,
}

fn dur_string(d: &Duration) -> String {
    if d.as_secs() != 0 {
        format!("{:.3} s", d.as_secs_f32())
    } else if d.as_millis() != 0 {
        format!("{} ms", d.as_micros() as f32 / 1000.0)
    } else if d.as_micros() != 0 {
        format!("{} us", d.as_nanos() as f32 / 1000.0)
    } else {
        format!("{} ns", d.as_nanos())
    }
}

impl AocResult {
    fn new(
        name: &str,
        gen_time: Duration,
        p1_time: Duration,
        e1: String,
        p2_time: Duration,
        e2: String,
    ) -> Self {
        Self {
            name: name.to_owned(),
            gen_time,
            p1_time,
            p2_time,
            e1,
            e2,
        }
    }
    pub fn print(&self) {
        println!(
            "{}\nGenerator in {}\n\tPart 1 in {}\n\t\t==> {}\n\tPart 2 in {}\n\t\t==> {}\n====> {}",
            self.name,
            dur_string(&self.gen_time),
            dur_string(&self.p1_time),
            self.e1,
            dur_string(&self.p2_time),
            self.e2,
            dur_string(&self.dur())
        );
    }

    pub fn print_line(&self, _avg_s1: &Duration, _avg_s2: &Duration, _avg: &Duration) {
        println!(
            "{:>20} {:>10} {:>15} {:>15} {:>15}",
            self.name,
            " ",
            self.e1,
            self.e2,
            dur_string(&self.dur())
        );
    }

    pub fn dur(&self) -> Duration {
        self.gen_time + self.p1_time + self.p2_time
    }
    pub fn durg(&self) -> Duration {
        self.gen_time
    }
    pub fn dur1(&self) -> Duration {
        self.p1_time
    }
    pub fn dur2(&self) -> Duration {
        self.p2_time
    }
    pub fn print_header() {
        println!(
            "{:>20} {:>10} {:>15} {:>15} {:>15}",
            "Day", "Solution", "Part A", "Part B", "Duration"
        );
    }
}

pub fn execute(day: Option<usize>) {
    if let Some(day) = day {
        let res = run(day);
        match res {
            Some(res) => res.print(),
            None => println!("Solution or file for day {} not found", day),
        }
    } else {
        let results = (1..=25).map(run).collect::<Vec<Option<AocResult>>>();
        let l = results.iter().flatten().count() as f64;
        if l == 0.0 {
            return;
        }
        let (dg, d1, d2, dt) = results.iter().flatten().fold(
            (
                Duration::default(),
                Duration::default(),
                Duration::default(),
                Duration::default(),
            ),
            |(ag, a1, a2, at), aoc| {
                (
                    ag + aoc.durg(),
                    a1 + aoc.dur1(),
                    a2 + aoc.dur2(),
                    at + aoc.dur(),
                )
            },
        );
        let (_ag, a1, a2, at) = (
            Duration::from_secs_f64(dg.as_secs_f64() / l),
            Duration::from_secs_f64(d1.as_secs_f64() / l),
            Duration::from_secs_f64(d2.as_secs_f64() / l),
            Duration::from_secs_f64(dt.as_secs_f64() / l),
        );

        AocResult::print_header();
        for result in results.into_iter().flatten() {
            result.print_line(&a1, &a2, &at);
        }
        println!("===> {} for everything", dur_string(&dt))
    }
}

#[allow(dead_code)]
fn run_day_mut<A, G, T1, T2, F1, F2>(name: &str, input: &str, gen: G, fn1: F1, fn2: F2) -> AocResult
where
    T1: Display,
    T2: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: FnMut(&'_ mut A) -> T1,
    F2: FnMut(&'_ mut A) -> T2,
{
    let (gen_time, mut input) = evaluate_function(input, gen);
    let p1 = evaluate_function(&mut input.clone(), fn1);
    let p2 = evaluate_function(&mut input, fn2);
    AocResult::new(
        name,
        gen_time,
        p1.0,
        format!("{}", p1.1),
        p2.0,
        format!("{}", p2.1),
    )
}

#[allow(dead_code)]
fn run_day<A, G, T1, T2, F1, F2>(name: &str, input: &str, gen: G, fn1: F1, fn2: F2) -> AocResult
where
    T1: Display,
    T2: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: Fn(&A) -> T1,
    F2: Fn(&A) -> T2,
{
    let (gen_time, input) = evaluate_function(input, gen);
    let p1 = evaluate_function(&input, fn1);
    let p2 = evaluate_function(&input, fn2);
    AocResult::new(
        name,
        gen_time,
        p1.0,
        format!("{}", p1.1),
        p2.0,
        format!("{}", p2.1),
    )
}

fn evaluate_function<A, T, F: FnMut(A) -> T>(input: A, mut func: F) -> (Duration, T) {
    let now = Instant::now();
    let result = func(input);
    (Instant::now() - now, result)
}

trait Pretty {
    fn pretty(&self) -> String;
}

impl Pretty for Duration {
    fn pretty(&self) -> String {
        if self.as_secs() != 0 {
            format!("{} s", self.as_secs())
        } else if self.as_millis() != 0 {
            format!("{} ms", self.as_millis())
        } else if self.as_micros() != 0 {
            format!("{} us", self.as_micros())
        } else {
            format!("{} ns", self.as_nanos())
        }
    }
}
