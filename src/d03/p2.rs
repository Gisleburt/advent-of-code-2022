use std::str::FromStr;
use crate::domain::rucksack::{GroupRucksacks, Rucksack};
use crate::input::string_iter::StringIter;

pub fn run() -> String {
    let stdin = std::io::stdin();
    let input = StringIter::<String, _>::from(stdin.lock());
    let mut rucksacks = input.map(|r| Rucksack::from_str(&r).unwrap()).peekable();
    let mut groups = vec![];
    while rucksacks.peek().is_some() {
        groups.push(GroupRucksacks::from(rucksacks.by_ref().take(3).collect::<Vec<_>>()))
    }

    let sum: usize = groups
        .iter()
        .map(|rs| rs.find_badge().priority())
        .sum();
    format!("{}", sum)
}
