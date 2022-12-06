use crate::input::grouped_input::GroupedInput;
use std::io::BufRead;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = GroupedInput::from(buf_read);
    let max = input
        .map(|pack| pack.iter().sum())
        .reduce(usize::max)
        .expect("Something went wrong, there were no numbers");
    format!("{}", max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "24000")
    }
}
