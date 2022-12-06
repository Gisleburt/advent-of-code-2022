use std::fmt::Debug;
use std::io::BufRead;
use std::marker::PhantomData;
use std::str::FromStr;

/// Takes a buffer and returns it one line at a time
pub struct StringIter<F, R: BufRead> {
    read: R,
    phantom_data: PhantomData<F>,
}

/// Converts any BufRead to a Grouped Input.
///
/// Note, unless Rust can infer `F`, you'll need to specify it
///
/// ```rust
/// use std::io::Cursor;
/// use advent_of_code_2022::input::string_iter::StringIter;
///
/// let raw = Cursor::new("123\n456\n\n789");
/// let mut input = StringIter::<usize, _>::from(raw);
/// ```
impl<F, R: BufRead> From<R> for StringIter<F, R> {
    fn from(read: R) -> Self {
        StringIter {
            read,
            phantom_data: PhantomData,
        }
    }
}

/// Reads the buffer one line at a time
///
/// ```rust
/// use std::io::Cursor;
/// use advent_of_code_2022::input::string_iter::StringIter;
///
/// let raw = Cursor::new("123\n456\n789");
/// let mut input = StringIter::from(raw);
/// assert_eq!(input.next(), Some(123));
/// assert_eq!(input.next(), Some(456));
/// assert_eq!(input.next(), Some(789));
/// assert_eq!(input.next(), None);
/// ```
impl<F: FromStr, R: BufRead> Iterator for StringIter<F, R>
where
    F::Err: Debug,
{
    type Item = F; // ToDo: Can we nest an iterator so we don't have to alloc here

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::with_capacity(8);
        self.read
            .read_line(&mut buffer)
            .expect("Failed to read from buffer");
        let trimmed = buffer.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.parse().expect("Could not parse string"))
        }
    }
}
