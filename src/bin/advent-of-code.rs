use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    D01p1,
    D01p2,
    D02p1,
    D02p2,
}

fn main() {
    let opt = Opt::from_args();
    let output = match opt {
        Opt::D01p1 => advent_of_code_2022::d01::p1::run(),
        Opt::D01p2 => advent_of_code_2022::d01::p2::run(),
        Opt::D02p1 => advent_of_code_2022::d02::p1::run(),
        Opt::D02p2 => advent_of_code_2022::d02::p2::run(),
    };
    println!("{}", output)
}
