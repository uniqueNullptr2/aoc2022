use criterion::{criterion_main, Criterion, criterion_group};



pub fn day1(_c: &mut Criterion) {
    // c.bench_function("aoc2021_day01_gen",|b| {
    //     b.iter_batched(|| {
    //         std::fs::read_to_string("input/2021/day1.txt").unwrap()
    //     }, |s| aoc1::generate_1(&s), criterion::BatchSize::SmallInput)
    // });

    // c.bench_function("aoc2021_day01_solve1",|b| {
    //     b.iter_batched(|| {
    //         let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
    //         aoc1::generate_1(&s)
    //     }, |s| aoc1::solve_part1(&s), criterion::BatchSize::SmallInput)
    // });

    // c.bench_function("aoc2021_day01_solve2",|b| {
    //     b.iter_batched(|| {
    //         let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
    //         aoc1::generate_1(&s)
    //     }, |s| aoc1::solve_part2(&s), criterion::BatchSize::SmallInput)
    // });
}


criterion_group!( aoc2022,  day1);

criterion_main!(aoc2022);
