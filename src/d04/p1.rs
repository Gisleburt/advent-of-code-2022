use crate::domain::search_party::SearchPair;
use crate::input::string_iter::StringIter;
use std::io::BufRead;
use std::str::FromStr;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = StringIter::<String, _>::from(buf_read);
    let sum = input
        .map(|line| SearchPair::from_str(&line).unwrap())
        .filter(|sp| sp.contains_complete_overlap())
        .count();
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
        assert_eq!(&output, "2");
    }
}
