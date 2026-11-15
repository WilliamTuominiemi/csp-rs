#[derive(Copy, Clone)]
struct Slot<const MTU: usize> {
    buffer: [u8; MTU],
    occupied: bool,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub struct BufferIndex(usize);

pub struct Pool<const N: usize, const MTU: usize> {
    slots: [Slot<MTU>; N],
}

impl<const N: usize, const MTU: usize> Pool<N, MTU> {
    pub const fn new() -> Pool<N, MTU> {
        let slot = Slot {
            buffer: [0; MTU],
            occupied: false,
            len: 0,
        };
        let slots = [slot; N];
        Pool { slots }
    }

    pub fn allocate(&mut self) -> Option<BufferIndex> {
        for i in 0..self.slots.len() {
            let slot = &mut self.slots[i];
            if !slot.occupied {
                slot.occupied = true;
                return Some(BufferIndex(i));
            }
        }

        None
    }

    pub fn release(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_and_release() {
        let mut pool: Pool<3, 8> = Pool::new();

        assert!(!pool.slots[0].occupied);
        assert!(!pool.slots[1].occupied);
        assert!(!pool.slots[2].occupied);

        let first = pool.allocate();

        assert!(pool.slots[0].occupied);
        assert!(!pool.slots[1].occupied);
        assert!(!pool.slots[2].occupied);
        assert_eq!(first, Some(BufferIndex(0)));

        let second = pool.allocate();

        assert!(pool.slots[0].occupied);
        assert!(pool.slots[1].occupied);
        assert!(!pool.slots[2].occupied);
        assert_eq!(second, Some(BufferIndex(1)));

        let third = pool.allocate();

        assert!(pool.slots[0].occupied);
        assert!(pool.slots[1].occupied);
        assert!(pool.slots[2].occupied);
        assert_eq!(third, Some(BufferIndex(2)));

        let out_of_memory = pool.allocate();
        assert_eq!(out_of_memory, None);
    }
}
