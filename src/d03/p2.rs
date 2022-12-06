use crate::domain::rucksack::{GroupRucksacks, Rucksack};
use crate::input::string_iter::StringIter;
use std::io::BufRead;
use std::str::FromStr;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = StringIter::<String, _>::from(buf_read);
    let mut rucksacks = input.map(|r| Rucksack::from_str(&r).unwrap()).peekable();
    let mut groups = vec![];
    while rucksacks.peek().is_some() {
        groups.push(GroupRucksacks::from(
            rucksacks.by_ref().take(3).collect::<Vec<_>>(),
        ))
    }

    let sum: usize = groups.iter().map(|rs| rs.find_badge().priority()).sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "70")
    }
}
