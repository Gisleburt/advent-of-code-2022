use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    D01p1,
    D01p2,
    D02p1,
    D02p2,
    D03p1,
    D03p2,
    D04p1,
    D04p2,
    D05p1,
}

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let opt = Opt::from_args();
    let output = match opt {
        Opt::D01p1 => advent_of_code_2022::d01::p1::run(lock),
        Opt::D01p2 => advent_of_code_2022::d01::p2::run(lock),
        Opt::D02p1 => advent_of_code_2022::d02::p1::run(lock),
        Opt::D02p2 => advent_of_code_2022::d02::p2::run(lock),
        Opt::D03p1 => advent_of_code_2022::d03::p1::run(lock),
        Opt::D03p2 => advent_of_code_2022::d03::p2::run(lock),
        Opt::D04p1 => advent_of_code_2022::d04::p1::run(lock),
        Opt::D04p2 => advent_of_code_2022::d04::p2::run(lock),
        Opt::D05p1 => advent_of_code_2022::d05::p1::run(lock),
    };
    println!("{}", output)
}
