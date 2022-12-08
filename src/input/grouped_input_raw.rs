use std::io::BufRead;

/// Takes a buffer and breaks it into parts based on blank lines
pub struct GroupedInputRaw<R: BufRead>(R);

/// Converts any BufRead to a Grouped Input.
///
/// Note, unless Rust can infer `F`, you'll need to specify it
///
/// ```rust
/// use std::io::Cursor;
/// use advent_of_code_2022::input::grouped_input_raw::GroupedInputRaw;
///
/// let raw = Cursor::new("123\n456\n\n789");
/// let mut input = GroupedInputRaw::from(raw);
/// ```
impl<R: BufRead> From<R> for GroupedInputRaw<R> {
    fn from(read: R) -> Self {
        GroupedInputRaw(read)
    }
}

/// Reads the buffer into groups of Vec's breaking on blank lines
///
/// ```rust
/// use std::io::Cursor;
/// use advent_of_code_2022::input::grouped_input_raw::GroupedInputRaw;
///
/// let raw = Cursor::new("123\n456\n\n789");
/// let mut input = GroupedInputRaw::from(raw);
/// assert_eq!(input.next(), Some(vec!["123".to_string(), "456".to_string()]));
/// assert_eq!(input.next(), Some(vec!["789".to_string()]));
/// assert_eq!(input.next(), None);
/// ```
impl<R: BufRead> Iterator for GroupedInputRaw<R> {
    type Item = Vec<String>; // ToDo: Can we nest an iterator so we don't have to alloc here

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();

        loop {
            let mut buffer = String::with_capacity(8);
            self.0
                .read_line(&mut buffer)
                .expect("Failed to read from buffer");
            let trimmed = buffer.trim();
            if trimmed.is_empty() {
                break;
            }
            let line = buffer
                .strip_suffix("\r\n")
                .or_else(|| buffer.strip_suffix('\n'))
                .unwrap_or(&buffer)
                .to_string();
            group.push(line);
        }

        if group.is_empty() {
            None
        } else {
            Some(group)
        }
    }
}
