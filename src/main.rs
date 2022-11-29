use aoc::execute;
use clap::Parser;


mod aoc;
mod day01;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    day: Option<usize>
}
fn main() {
    let args = Args::parse();

    execute(args.day)
}