use advent_of_code_2022::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    D01p1,
    D01p2,
    D02p1,
}

fn main() {
    let opt = Opt::from_args();
    let output = match opt {
        Opt::D01p1 => d01p1(),
        Opt::D01p2 => d01p2(),
        Opt::D02p1 => d02p1(),
    };
    println!("{}", output)
}
