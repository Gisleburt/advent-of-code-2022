use std::str::FromStr;
use crate::domain::rucksack::Rucksack;
use crate::input::string_iter::StringIter;

pub fn run() -> String {
    let stdin = std::io::stdin();
    let input = StringIter::<String, _>::from(stdin.lock());
    let rucksacks: Result<Vec<_>, _> = input.map(|r| Rucksack::from_str(&r)).collect();
    let sum: usize = rucksacks
        .unwrap()
        .iter()
        .map(|rs| rs.clashing_priority_value())
        .sum();
    format!("{}", sum)
}
