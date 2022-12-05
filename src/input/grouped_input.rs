use std::fmt::Debug;
use std::io::BufRead;
use std::marker::PhantomData;
use std::str::FromStr;

/// Takes a buffer and breaks it into parts based on blank lines
pub struct GroupedInput<F, R: BufRead> {
    read: R,
    phantom_data: PhantomData<F>,
}

/// Converts any BufRead to a Grouped Input
impl<F, R: BufRead> From<R> for GroupedInput<F, R> {
    fn from(read: R) -> Self {
        GroupedInput {
            read,
            phantom_data: PhantomData,
        }
    }
}

/// Reads the buffer into groups of Vec's breaking on blank lines
impl<F: FromStr, R: BufRead> Iterator for GroupedInput<F, R>
where
    F::Err: Debug,
{
    type Item = Vec<F>; // ToDo: Can we nest an iterator so we don't have to alloc here

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();

        loop {
            let mut buffer = String::with_capacity(8);
            self.read
                .read_line(&mut buffer)
                .expect("Failed to read from buffer");
            let trimmed = buffer.trim();
            if trimmed.is_empty() {
                break;
            }
            group.push(trimmed.parse().expect("Input data was invalid"))
        }

        if group.is_empty() {
            None
        } else {
            Some(group)
        }
    }
}
