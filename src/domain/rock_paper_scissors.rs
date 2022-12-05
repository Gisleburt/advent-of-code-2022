use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum GameResult {
    Win(RockPaperScissors),
    Draw(RockPaperScissors),
    Loss(RockPaperScissors),
}

impl GameResult {
    pub fn score(&self) -> usize {
        match self {
            GameResult::Win(play) => 6 + play.score(),
            GameResult::Draw(play) => 3 + play.score(),
            GameResult::Loss(play) => play.score(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn score(&self) -> usize {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    pub fn compare(&self, other: RockPaperScissors) -> GameResult {
        match self {
            RockPaperScissors::Rock => match other {
                RockPaperScissors::Rock => GameResult::Draw(*self),
                RockPaperScissors::Paper => GameResult::Loss(*self),
                RockPaperScissors::Scissors => GameResult::Win(*self),
            }
            RockPaperScissors::Paper => match other {
                RockPaperScissors::Rock => GameResult::Win(*self),
                RockPaperScissors::Paper => GameResult::Draw(*self),
                RockPaperScissors::Scissors => GameResult::Loss(*self),
            }
            RockPaperScissors::Scissors => match other {
                RockPaperScissors::Rock => GameResult::Loss(*self),
                RockPaperScissors::Paper => GameResult::Win(*self),
                RockPaperScissors::Scissors => GameResult::Draw(*self),
            }
        }
    }
}

impl TryFrom<char> for RockPaperScissors {
    type Error = String;
    fn try_from(c: char) -> Result<RockPaperScissors, Self::Error> {
        match c {
            'A' | 'X' => Ok(RockPaperScissors::Rock),
            'B' | 'Y' => Ok(RockPaperScissors::Paper),
            'C' | 'Z' => Ok(RockPaperScissors::Scissors),
            _ => Err(format!("'{}' is not a valid option for Rock Paper Scissors", c)),
        }
    }
}

pub struct RockPaperScissorsGame {
    theirs: RockPaperScissors,
    mine: RockPaperScissors,
}

impl FromStr for RockPaperScissorsGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() > 3 {
            Err(format!("Invalid string length, expect 3 go {}: '{}'", s.len(), s))
        }
        else if s.chars().nth(1) != Some(' ') {
            Err(format!("Invalid string, expected space in center, got {}", s))
        }
        else {
            let theirs = s.chars().next().unwrap().try_into()?;
            let mine = s.chars().nth(2).unwrap().try_into()?;
            Ok(Self {
                theirs,
                mine,
            })
        }
    }
}

impl RockPaperScissorsGame {
    pub fn score(&self) -> usize {
        self.mine.compare(self.theirs).score()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::input::string_iter::StringIter;
    use super::*;

    #[test]
    fn test_d02p1_example() {
        let input = Cursor::new("A Y\nB X\nC Z");
        let input = StringIter::<String, _>::from(input);
        let score: usize = input
            .map(|s| RockPaperScissorsGame::from_str(&s).unwrap())
            .map(|g| g.score())
            .sum();
        assert_eq!(score, 15)
    }
}
