use std::str::FromStr;
use crate::domain::rock_paper_scissors::RockPaperScissorsGame;
use crate::input::string_iter::StringIter;

pub fn d02p1() -> String {
    let stdin = std::io::stdin();
    let input = StringIter::<String, _>::from(stdin.lock());
    let score: usize = input
        .map(|s| RockPaperScissorsGame::from_str(&s).unwrap())
        .map(|g| g.score())
        .sum();
    format!("{}", score)
}
