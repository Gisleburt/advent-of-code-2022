use advent_of_code_2022::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    D01p1,
}

fn main() {
    let opt = Opt::from_args();
    let output = match opt {
        Opt::D01p1 => d01p1(),
    };
    println!("{}", output)
}
