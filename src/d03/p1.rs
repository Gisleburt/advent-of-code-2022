use std::io::BufRead;
use crate::domain::rucksack::Rucksack;
use crate::input::string_iter::StringIter;
use std::str::FromStr;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = StringIter::<String, _>::from(buf_read);
    let rucksacks: Result<Vec<_>, _> = input.map(|r| Rucksack::from_str(&r)).collect();
    let sum: usize = rucksacks
        .unwrap()
        .iter()
        .map(|rs| rs.clashing_priority_value())
        .sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_run() {
        let input = Cursor::new("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let output = run(input);
        assert_eq!(&output, "157");
    }
}
