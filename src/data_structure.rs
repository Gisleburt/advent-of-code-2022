use std::collections::VecDeque;
use std::ops::Add;

// This file mostly copied from 2021

pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    /// Creates a buffer with a given capacity
    ///
    /// Note: Internally a VecDeque is used which _may_ reserve more memory than will be used.
    ///
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let ring_buffer = RingBuffer::<usize>::with_capacity(2);
    /// assert_eq!(ring_buffer.capacity(), 2);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        RingBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Get the capacity of the ring buffer. This value does not change
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.capacity(), 2);
    /// ring_buffer.push(1);
    /// ring_buffer.push(2);
    /// ring_buffer.push(3);
    /// assert_eq!(ring_buffer.capacity(), 2);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the length of the ring buffer (how much data is currently in the buffer). This can not
    /// go over the capacity of the buffer.
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.len(), 0);
    /// ring_buffer.push(1);
    /// assert_eq!(ring_buffer.len(), 1);
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.len(), 2);
    /// ring_buffer.push(3);
    /// assert_eq!(ring_buffer.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Tells you if the ring buffer is empty
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.is_empty(), true);
    /// ring_buffer.push(1);
    /// assert_eq!(ring_buffer.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Determines if the ring buffer is full. Once full, pushing new data to the buffer overwrites
    /// the oldest piece of data
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.is_full(), false);
    /// ring_buffer.push(1);
    /// assert_eq!(ring_buffer.is_full(), false);
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.is_full(), true);
    /// ring_buffer.push(3);
    /// assert_eq!(ring_buffer.is_full(), true);
    /// ```
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Add data to the ring buffer
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.is_full(), false);
    /// ring_buffer.push(1);
    /// assert_eq!(ring_buffer.is_full(), false);
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.is_full(), true);
    /// ring_buffer.push(3);
    /// assert_eq!(ring_buffer.is_full(), true);
    /// ```
    pub fn push(&mut self, value: T) {
        if self.is_full() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }
}

impl<T: PartialEq> RingBuffer<T> {
    /// Checks to see if there are any duplicated in the ring
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// ring_buffer.push(1);
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.contains_duplicates(), false);
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.contains_duplicates(), true);
    /// ```
    pub fn contains_duplicates(&self) -> bool {
        self.buffer.iter().enumerate().any(|(i, v1)| {
            self.buffer.iter().skip(i+1).any(|v2| v1 == v2)
        })
    }
}

impl<T: Add<Output=T> + Default + Copy> RingBuffer<T> {
    /// For data types that can be added together, this will get the total value of the buffer
    /// ```rust
    /// use advent_of_code_2022::data_structure::RingBuffer;
    ///
    /// let mut ring_buffer = RingBuffer::with_capacity(2);
    /// assert_eq!(ring_buffer.total(), 0); // [] = 0
    /// ring_buffer.push(1);
    /// assert_eq!(ring_buffer.total(), 1); // [1] = 1
    /// ring_buffer.push(2);
    /// assert_eq!(ring_buffer.total(), 3); // [1, 2] = 3
    /// ring_buffer.push(3);
    /// assert_eq!(ring_buffer.total(), 5); // [2, 3] = 5
    /// ```
    pub fn total(&self) -> T {
        self.buffer.iter().fold(T::default(), |acc, cur| acc + *cur)
    }
}
