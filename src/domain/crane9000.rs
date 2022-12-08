use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Move {
    type Err = String;

    /// Parses a string instruction
    /// ```rust
    /// use std::str::FromStr;
    /// use advent_of_code_2022::domain::crane9000::Move;
    ///
    /// # fn main() -> Result<(), String> {
    /// let m = Move::from_str("move 1 from 2 to 3")?;
    /// assert_eq!(m.amount, 1);
    /// assert_eq!(m.from, 2);
    /// assert_eq!(m.to, 3);
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let caps = MOVE_RE
            .captures(s)
            .ok_or_else(|| format!("String not a move instruction: {}", s))?;
        Ok(Self {
            amount: caps.get(1).unwrap().as_str().parse().unwrap(),
            from: caps.get(2).unwrap().as_str().parse().unwrap(),
            to: caps.get(3).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Move(Move),
}

impl From<Move> for Instruction {
    fn from(m: Move) -> Self {
        Self::Move(m)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Move::from_str(s)?.into())
    }
}

pub type Crate = char;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Stack(Vec<Crate>);

impl Deref for Stack {
    type Target = [Crate];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Stack {
    /// Take the top `n` items but keep the order the same
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stack;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut stack = Stack::from(vec!['1', '2', '3', '4', '5', '6']);
    /// assert_eq!(stack.take(3), vec!['6', '5', '4']);
    /// # Ok(())
    /// # }
    /// ```
    pub fn take(&mut self, amount: usize) -> Vec<Crate> {
        let mut removed = Vec::with_capacity(amount);
        let new_len = self.0.len().saturating_sub(amount);
        for removed_element in self.0.drain(new_len..) {
            removed.push(removed_element);
        }
        removed.reverse();
        removed
    }

    /// Places items on top of the stack
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stack;
    /// use std::ops::Deref;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut stack = Stack::from(vec!['1', '2', '3']);
    /// stack.place(vec!['4', '5', '6']);
    /// assert_eq!(stack.deref(), &['1', '2', '3', '4', '5', '6']);
    /// # Ok(())
    /// # }
    /// ```
    pub fn place(&mut self, mut crates: Vec<Crate>) {
        self.0.append(&mut crates)
    }

    /// Gets the crate on the top of the stack without removing it
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stack;
    /// use std::ops::Deref;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut stack = Stack::from(vec!['1', '2', '3']);
    /// assert_eq!(stack.top_crate(), '3');
    /// assert_eq!(stack.deref(), &['1', '2', '3']);
    /// # Ok(())
    /// # }
    /// ```
    pub fn top_crate(&self) -> Crate {
        self.0.last().copied().unwrap_or(' ')
    }
}

impl From<Vec<Crate>> for Stack {
    fn from(v: Vec<Crate>) -> Self {
        Self(v)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stacks(Vec<Stack>);

impl Stacks {
    /// Gets the message from the top crate of each stack
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::{Move, Stack, Stacks};
    /// use std::ops::Deref;
    ///
    /// # fn main() -> Result<(), String> {
    /// let mut stacks = Stacks::from(vec![
    ///     Stack::from(vec!['y', 's', 'e']),
    ///     Stack::from(vec![]),
    /// ]);
    ///
    /// stacks.instruct(Move {
    ///     from: 1,
    ///     to: 2,
    ///     amount: 2,
    /// });
    ///
    /// assert_eq!(stacks, Stacks::from(vec![
    ///     Stack::from(vec!['y']),
    ///     Stack::from(vec!['e', 's']),
    /// ]));
    /// # Ok(())
    /// # }
    /// ```
    pub fn instruct<I: Into<Instruction>>(&mut self, instruction: I) {
        match instruction.into() {
            Instruction::Move(m) => {
                // TODO: This will panic if to or from are out of bounds
                let crates = self.0[m.from - 1].take(m.amount);
                self.0[m.to - 1].place(crates);
            }
        }
    }

    /// Gets the message from the top crate of each stack
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stack;
    /// use std::ops::Deref;
    ///
    /// # fn main() -> Result<(), String> {
    /// use advent_of_code_2022::domain::crane9000::Stacks;
    /// let stacks = vec![
    ///     Stack::from(vec!['f', 'g', 'h']),
    ///     Stack::from(vec!['d', 'e']),
    ///     Stack::from(vec!['f', 'g', 'h', 'i', 'j', 'k', 'l']),
    ///     Stack::from(vec!['l']),
    ///     Stack::from(vec!['m', 'n', 'o']),
    /// ];
    /// let stacks = Stacks::from(stacks);
    /// assert_eq!(stacks.get_message(), "hello");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_message(&self) -> String {
        self.0.iter().map(|stack| stack.top_crate()).collect()
    }

    /// Returns the size of the largest stack
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stack;
    ///
    /// # fn main() -> Result<(), String> {
    /// use advent_of_code_2022::domain::crane9000::Stacks;
    /// let stacks = vec![
    ///     Stack::from(vec!['f', 'g', 'h']),
    ///     Stack::from(vec!['d', 'e']),
    ///     Stack::from(vec!['f', 'g', 'h', 'i', 'j', 'k', 'l']),
    /// ];
    /// let stacks = Stacks::from(stacks);
    /// assert_eq!(stacks.largest_stack_size(), 7);
    /// # Ok(())
    /// # }
    /// ```
    pub fn largest_stack_size(&self) -> usize {
        self.0.iter().fold(0, |acc, cur| max(acc, cur.0.len()))
    }
}

impl From<Vec<Stack>> for Stacks {
    fn from(stacks: Vec<Stack>) -> Self {
        Self(stacks)
    }
}

impl From<Vec<String>> for Stacks {
    /// Converts a string into stacks
    /// ```rust
    /// use advent_of_code_2022::domain::crane9000::Stacks;
    /// let input = vec![
    ///     "    [O]".to_string(),
    ///     "[W] [C]".to_string(),
    ///     "[Z] [M] [O]".to_string(),
    ///     " 1   2   3 ".to_string(),
    /// ];
    /// let stacks = Stacks::from(input);
    /// assert_eq!(stacks.get_message(), "WOO".to_string());
    /// ```
    fn from(mut strings: Vec<String>) -> Self {
        strings.reverse();
        let mut iter = strings.into_iter();
        let first = iter.next().unwrap();
        let len = (first.len() + 1) / 4;
        let mut stacks = vec![Stack::default(); len];

        for string in iter {
            let crates: Vec<String> = string
                .chars()
                .chunks(4)
                .into_iter()
                .map(|chunk| chunk.collect())
                .collect();
            for (i, string) in crates.iter().enumerate() {
                let c = string.chars().nth(1).unwrap();
                match c {
                    'A'..='Z' | 'a'..='z' => stacks[i].0.push(c),
                    _ => {}
                }
            }
        }

        Self(stacks)
    }
}
