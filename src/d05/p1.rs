use crate::domain::crane9000::{Instruction, Stacks};
use crate::input::grouped_input_raw::GroupedInputRaw;
use std::io::BufRead;
use std::str::FromStr;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let mut input = GroupedInputRaw::from(buf_read);
    let crates = input.next().expect("No crates found");
    let mut stacks = Stacks::from(crates);
    input
        .next()
        .expect("No instructions found")
        .into_iter()
        .filter(|string| !string.is_empty())
        .map(|string| Instruction::from_str(&string).unwrap())
        .for_each(|instruction| stacks.instruct(instruction));

    stacks.get_message()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "CMZ");
    }
}
