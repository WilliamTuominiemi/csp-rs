pub struct CircularBuffer<T, const CAP: usize> {
    data: [Option<T>; CAP],
    head: usize,
    tail: usize,
    size: usize,
}

impl<T, const CAP: usize> CircularBuffer<T, CAP> {
    pub fn new() -> Self {
        assert!(CAP > 0, "CircularBuffer capacity must be > 0");

        Self {
            data: [const { None }; CAP],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == CAP
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        self.data[self.tail] = Some(value);
        self.tail = (self.tail + 1) % CAP;
        self.size += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.data[self.head].take().unwrap();
        self.head = (self.head + 1) % CAP;
        self.size -= 1;

        Some(value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.data[self.head].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let buffer: CircularBuffer<usize, 4> = CircularBuffer::new();

        assert!(buffer.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut buffer: CircularBuffer<usize, 2> = CircularBuffer::new();

        let _ = buffer.push(1);
        let _ = buffer.push(1);

        assert!(!buffer.is_empty());
        assert!(buffer.is_full());
    }

    #[test]
    fn test_get_size() {
        let mut buffer: CircularBuffer<usize, 2> = CircularBuffer::new();

        let _ = buffer.push(1);
        let _ = buffer.push(1);

        assert_eq!(buffer.get_size(), 2);
    }

    #[test]
    fn test_push() {
        let mut buffer: CircularBuffer<usize, 4> = CircularBuffer::new();

        let _ = buffer.push(4);
        let _ = buffer.push(3);

        assert_eq!(buffer.data[0], Some(4));
        assert_eq!(buffer.data[1], Some(3));
    }

    #[test]
    fn test_push_on_full() {
        let mut buffer: CircularBuffer<usize, 2> = CircularBuffer::new();

        let _ = buffer.push(4);
        let _ = buffer.push(3);

        assert_eq!(buffer.push(3), Err(3));

        assert_eq!(buffer.data[0], Some(4));
        assert_eq!(buffer.data[1], Some(3));
    }

    #[test]
    fn test_pop() {
        let mut buffer: CircularBuffer<usize, 4> = CircularBuffer::new();

        let _ = buffer.push(5);
        let _ = buffer.push(6);

        assert_eq!(buffer.pop(), Some(5));
        assert_eq!(buffer.pop(), Some(6));
    }

    #[test]
    fn test_pop_on_empty() {
        let mut buffer: CircularBuffer<usize, 4> = CircularBuffer::new();

        assert!(buffer.pop().is_none());
    }

    #[test]
    fn test_peek() {
        let mut buffer: CircularBuffer<usize, 4> = CircularBuffer::new();

        let _ = buffer.push(7);
        let _ = buffer.push(8);

        assert_eq!(buffer.peek(), Some(7).as_ref());
        assert_eq!(buffer.peek(), Some(7).as_ref());
    }

    fn test_wrap_around() {
        let mut buffer: CircularBuffer<usize, 3> = CircularBuffer::new();

        let _ = buffer.push(7);
        let _ = buffer.push(5);
        let _ = buffer.push(8);
        assert_eq!(buffer.pop(), Some(8));
        assert_eq!(buffer.pop(), Some(5));
        let _ = buffer.push(6);
        let _ = buffer.push(9);
        assert_eq!(buffer.pop(), Some(9));
        assert_eq!(buffer.pop(), Some(6));
        assert_eq!(buffer.pop(), Some(7));
    }
}
