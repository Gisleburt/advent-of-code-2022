use std::io::BufRead;

pub fn run<R: BufRead>(_buf_read: R) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::io::Cursor;
    //
    // #[test]
    // fn test_run() {
    //     let input = Cursor::new(include_str!("test-input.txt"));
    //     let output = run(input);
    //     assert_eq!(&output, "CMZ");
    // }
}
