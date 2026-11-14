#[derive(PartialEq, Debug)]
pub struct Header {
    pub priority: u8,
    pub source: u8,
    pub destination: u8,
    pub destination_port: u8,
    pub source_port: u8,
    pub flags: u8,
}

impl Header {
    pub fn from_bytes(bytes: [u8; 4]) -> Header {
        let raw = u32::from_be_bytes(bytes);

        Header {
            priority: ((raw >> 30) & 0x03) as u8,
            source: ((raw >> 25) & 0x1F) as u8,
            destination: ((raw >> 20) & 0x1F) as u8,
            destination_port: ((raw >> 14) & 0x3F) as u8,
            source_port: ((raw >> 8) & 0x3F) as u8,
            flags: (raw & 0x0F) as u8,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let raw = ((self.priority as u32 & 0x03) << 30)
            | ((self.source as u32 & 0x1F) << 25)
            | ((self.destination as u32 & 0x1F) << 20)
            | ((self.destination_port as u32 & 0x3F) << 14)
            | ((self.source_port as u32 & 0x3F) << 8)
            | (self.flags as u32 & 0x0F);

        raw.to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let raw_header = [0x50, 0x11, 0xCC, 0x01];

        let expected_header = Header {
            priority: 1,
            source: 8,
            destination: 1,
            destination_port: 7,
            source_port: 12,
            flags: 0x01,
        };

        let decoded = Header::from_bytes(raw_header);
        assert_eq!(decoded, expected_header);
    }

    #[test]
    fn test_to_bytes() {
        let header = Header {
            priority: 1,
            source: 8,
            destination: 1,
            destination_port: 7,
            source_port: 12,
            flags: 0x01,
        };

        let expected_raw_header = [0x50, 0x11, 0xCC, 0x01];

        let encoded = header.to_bytes();
        assert_eq!(encoded, expected_raw_header);
    }
}
