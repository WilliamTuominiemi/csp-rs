#[derive(Copy, Clone)]
struct Slot<const MTU: usize> {
    buffer: [u8; MTU],
    occupied: bool,
    len: usize,
}

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

    pub fn allocate() {}

    pub fn release() {}
}
