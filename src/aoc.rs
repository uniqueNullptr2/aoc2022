use std::{time::{Instant, Duration}, fmt::Display};

use crate::day01;


fn run(day: usize) -> Option<AocResult>{
    std::fs::read_to_string(format!("inputs/day{:02}.txt", day)).ok().map(|s| {
        match day {
            1 => {
                Some(run_day_mut("AOC 2021 Day01", &s,
                    day01::parse,
                    |input| day01::part1(input),
                    |input| day01::part2(input)))
            },
            // 2 => {
            //     Some(self.run_day("AOC 2021 Day02", &s,
            //         aoc2::input_generator,
            //         |input| aoc2::solve_part1(input),
            //         |input| aoc2::solve_part2(input)))
            // },
            // 3 => {
            //     Some(self.run_day("AOC 2021 Day03", &s,
            //         aoc3::input_generator,
            //         |input| aoc3::solve_part1(input),
            //         |input| aoc3::solve_part2(input)))
            // },
            // 4 => {
            //     Some(self.run_day_mut("AOC 2021 Day04", &s,
            //         aoc4::input_generator,
            //         aoc4::solve_part1,
            //         aoc4::solve_part2))
            // },
            // 5 => {
            //     Some(self.run_day("AOC 2021 Day05", &s,
            //         aoc5::input_generator,
            //         |input| aoc5::solve_part1(input),
            //         |input| aoc5::solve_part2_arr(input)))
            // },
            // 6 => {
            //     Some(self.run_day_mut("AOC 2021 Day06", &s,
            //         aoc6::input_generator,
            //         aoc6::solve_part1,
            //         aoc6::solve_part2))
            // },
            // 7 => {
            //     Some(self.run_day("AOC 2021 Day07", &s,
            //         aoc7::input_generator,
            //         |input| aoc7::solve_part1(input),
            //         |input| aoc7::solve_part2(input)))
            // },
            // 8 => {
            //     Some(self.run_day("AOC 2021 Day08", &s,
            //         aoc8::input_generator,
            //         |input| aoc8::solve_part1(input),
            //         |input| aoc8::solve_part2(input)))
            // },
            // 9 => {
            //     Some(self.run_day("AOC 2021 Day09", &s,
            //         aoc9::input_generator,
            //         |input| aoc9::solve_part1(input),
            //         |input| aoc9::solve_part2(input)))
            // },
            // 10 => {
            //     Some(self.run_day("AOC 2021 Day10", &s,
            //         aoc10::input_generator,
            //         |input| aoc10::solve_part1(input),
            //         |input| aoc10::solve_part2(input)))
            // },
            // 11 => {
            //     Some(self.run_day_mut("AOC 2021 Day11", &s,
            //         aoc11::input_generator,
            //         |input| aoc11::solve_part1(input),
            //         |input| aoc11::solve_part2(input)))
            // },
            // 12 => {
            //     Some(self.run_day("AOC 2021 Day12", &s,
            //         aoc12::input_generator,
            //         aoc12::solve_part1,
            //         aoc12::solve_part2))
            // },
            // 13 => {
            //     Some(self.run_day_mut("AOC 2021 Day13", &s,
            //         aoc13::input_generator,
            //         aoc13::solve_part1,
            //         aoc13::solve_part2))
            // },
            // 14 => {
            //     Some(self.run_day("AOC 2021 Day14", &s,
            //         aoc14::input_generator,
            //         aoc14::solve_part1,
            //         aoc14::solve_part2))
            // },
            // 15 => {
            //     Some(self.run_day("AOC 2021 Day15", &s,
            //         aoc15::input_generator,
            //         aoc15::solve_part1,
            //         aoc15::solve_part2))
            // },
            // 16 => {
            //     Some(self.run_day("AOC 2021 Day16", &s,
            //         aoc16::input_generator,
            //         aoc16::solve_part1,
            //         aoc16::solve_part2))
            // },
            // 17 => {
            //     Some(self.run_day("AOC 2021 Day17", &s,
            //         aoc17::input_generator,
            //         aoc17::solve_part1,
            //         aoc17::solve_part2))
            // },
            // 18 => {
            //     Some(self.run_day("AOC 2021 Day18", &s,
            //         aoc18::input_generator,
            //         |input| aoc18::solve_part1(&input),
            //         |input| aoc18::solve_part2(&input)))
            // },
            _ => None
        }
    }).flatten()
}

pub struct AocResult {
    name: String,
    gen_time: Duration,
    p1_time: Duration,
    p2_time: Duration,
    e1: String,
    e2: String
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
    fn new(name: &str, gen_time: Duration, p1_time: Duration, e1: String, p2_time: Duration, e2: String) -> Self {
        Self{
            name: name.to_owned(),
            gen_time,
            p1_time,
            p2_time,
            e1,
            e2
        }
    }
    pub fn print(&self) {
        println!("{}\nGenerator in {}\n\tPart 1 in {}\n\t\t==> {}\n\tPart 2 in {}\n\t\t==> {}\n====> {}",
            self.name,
            dur_string(&self.gen_time),
            dur_string(&self.p1_time),
            self.e1,
            dur_string(&self.p2_time),
            self.e2,
            dur_string(&self.dur()));
    }

    pub fn print_line(&self, _avg_s1: &Duration, _avg_s2: &Duration, _avg: &Duration) {
        println!("{:>20} {:>10} {:>15} {:>15} {:>15}", self.name, " ", self.e1, self.e2, dur_string(&self.dur()));
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
        println!("{:>20} {:>10} {:>15} {:>15} {:>15}", "Day", "Solution", "Part A", "Part B", "Duration");
    }
}

pub fn execute(day: Option<usize>) {
    if let Some(day) = day {
        let res = run(day);
        match res {
            Some(res) => res.print(),
            None => println!("Solution or file for day {} not found", day)
        }
    } else {
        let results = (1..=25).map(|i| {
            run(i)
        }).collect::<Vec<Option<AocResult>>>();
        let l = results.iter().flatten().count() as f64;
        if l == 0.0 {
            return;
        }
        let (dg, d1, d2, dt) = results
            .iter()
            .flatten()
            .fold((Duration::default(),Duration::default(),Duration::default(),Duration::default()), |(ag,a1, a2, at),aoc| {
                (ag + aoc.durg(), a1 + aoc.dur1(), a2 + aoc.dur2(), at + aoc.dur())
            });
        let (_ag, a1, a2, at) = (Duration::from_secs_f64(dg.as_secs_f64() / l),
            Duration::from_secs_f64(d1.as_secs_f64() / l),
            Duration::from_secs_f64(d2.as_secs_f64() / l),
            Duration::from_secs_f64(dt.as_secs_f64() / l));

            AocResult::print_header();
            for result in results.into_iter().flatten() {
                result.print_line(&a1, &a2, &at);
            }
            println!("===> {} for everything", dur_string(&dt))
    }
}

#[allow(dead_code)]
fn run_day_mut<A, G, T1, T2, F1, F2>(name: &str, input: &str, gen: G, fn1: F1, fn2: F2) -> AocResult
where T1: Display,
    T2: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: FnMut(&'_ mut A) -> T1,
    F2: FnMut(&'_ mut A) -> T2
{
    let (gen_time,mut input) = evaluate_function(input, gen);
    let p1 = evaluate_function(&mut input.clone(), fn1);
    let p2 = evaluate_function(&mut input, fn2);
    AocResult::new(name, gen_time, p1.0, format!("{}", p1.1), p2.0, format!("{}", p2.1))
}

#[allow(dead_code)]
fn run_day<A, G, T1, T2, F1, F2>(name: &str, input: &str, gen: G, fn1: F1, fn2: F2) -> AocResult
where T1: Display,
    T2: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: Fn(&A) -> T1,
    F2: Fn(&A) -> T2
{
    let (gen_time,input) = evaluate_function(input, gen);
    let p1 = evaluate_function(&input, fn1);
    let p2 = evaluate_function(&input, fn2);
    AocResult::new(name, gen_time, p1.0, format!("{}", p1.1), p2.0, format!("{}", p2.1))
}


fn evaluate_function<A, T, F: FnMut(A) -> T>(input: A, mut func: F) -> (Duration, T) {
    let now = Instant::now();
    let result = func(input);
    (Instant::now()-now, result)
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
