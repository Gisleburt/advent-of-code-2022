use crate::input::grouped_input::GroupedInput;
use std::io::BufRead;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = GroupedInput::from(buf_read);
    let mut all_pack_values: Vec<usize> = input.map(|pack| pack.iter().sum()).collect();
    all_pack_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_3: usize = all_pack_values.iter().take(3).sum();
    format!("{}", top_3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "45000")
    }
}
