#![no_main]

use csp::header::Header;
use libfuzzer_sys::fuzz_target;

const USED_BITS: u32 = 0xFFFF_FF0F;

fuzz_target!(|data: &[u8]| {
    let Some(bytes) = data.get(..4) else { return };
    let bytes: [u8; 4] = bytes.try_into().unwrap();

    let header = Header::from_bytes(bytes);

    assert!(header.priority <= 0x03);
    assert!(header.source <= 0x1F);
    assert!(header.destination <= 0x1F);
    assert!(header.destination_port <= 0x3F);
    assert!(header.source_port <= 0x3F);
    assert!(header.flags <= 0x0F);

    let encoded = header.to_bytes();
    assert_eq!(
        u32::from_be_bytes(encoded),
        u32::from_be_bytes(bytes) & USED_BITS
    );

    assert_eq!(Header::from_bytes(encoded), header);
});
