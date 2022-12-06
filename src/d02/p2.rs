use crate::domain::rock_paper_scissors::RockPaperScissorsGame;
use crate::input::string_iter::StringIter;
use std::io::BufRead;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = StringIter::<String, _>::from(buf_read);
    let score: usize = input
        .map(|s| RockPaperScissorsGame::from_results_str(&s).unwrap())
        .map(|g| g.score())
        .sum();
    format!("{}", score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "12")
    }
}
