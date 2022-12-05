use std::io::BufRead;
use std::marker::PhantomData;

/// Takes a buffer and breaks it into parts based on blank lines
pub struct GroupedInput<T, R: BufRead>{
    read: R,
    phantom_data: PhantomData<T>
}

impl<R: BufRead> From<R> for GroupedInput<usize, R> {
    fn from(read: R) -> Self {
        GroupedInput {
            read,
            phantom_data: PhantomData
        }
    }
}

impl<R: BufRead> Iterator for GroupedInput<usize, R> {
    type Item = Vec<usize>; // ToDo: Can we nest an iterator so we don't have to alloc here

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
