use crate::data_structure::RingBuffer;
use crate::input::string_iter::StringIter;
use std::io::BufRead;

pub fn run<R: BufRead>(buf_read: R) -> String {
    let stream = StringIter::<String, _>::from(buf_read).next().unwrap();
    let data = stream.chars().enumerate();
    let mut buffer = RingBuffer::with_capacity(4);
    for (i, c) in data {
        buffer.push(c);
        if buffer.is_full() && !buffer.contains_duplicates() {
            return (i + 1).to_string();
        }
    }
    panic!("No marker found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_run() {
        let input = Cursor::new(include_str!("test-input.txt"));
        let output = run(input);
        assert_eq!(&output, "7");
    }

    #[test]
    fn additional_tests() {
        let input = Cursor::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(&run(input), "5");
        let input = Cursor::new("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(&run(input), "6");
        let input = Cursor::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(&run(input), "10");
        let input = Cursor::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(&run(input), "11");
    }
}
