use std::io::BufRead;
use crate::input::grouped_input::GroupedInput;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let input = GroupedInput::from(buf_read);
    let mut all_pack_values: Vec<usize> = input.map(|pack| pack.iter().sum()).collect();
    all_pack_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_3: usize = all_pack_values.iter().take(3).sum();
    format!("{}", top_3)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_run() {
        let input = Cursor::new("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        let output = run(input);
        assert_eq!(&output, "45000")
    }
}
