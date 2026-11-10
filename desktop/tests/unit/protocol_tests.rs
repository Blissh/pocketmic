#[cfg(test)]
mod tests {
    use pocketmic_daemon::net::protocol::*;

    #[test]
    fn test_encode_decode_valid_header() {
        let header = AudioPacketHeader::new(
            PacketType::Audio,
            42,
            1234567,
            HeaderFlags::MUTE | HeaderFlags::FEC,
            1,
            80,
        );

        let mut buf = [0u8; 16];
        header.encode(&mut buf).expect("Encode should succeed");

        let decoded = AudioPacketHeader::decode(&buf).expect("Decode should succeed");
        assert_eq!(header, decoded);
        assert!(decoded.is_muted());
        assert!(decoded.has_fec());
        assert!(!decoded.is_clipping());
    }

    #[test]
    fn test_reject_invalid_magic() {
        let mut buf = [0u8; 16];
        buf[0] = 0x12;
        buf[1] = 0x34; // Invalid magic
        buf[2] = 0x01; // Version 1

        let err = AudioPacketHeader::decode(&buf).expect_err("Should fail with invalid magic");
        assert_eq!(err, ProtocolError::InvalidMagic(0x1234));
    }

    #[test]
    fn test_reject_short_buffer() {
        let buf = [0u8; 10];
        let err = AudioPacketHeader::decode(&buf).expect_err("Should fail with short buffer");
        assert_eq!(err, ProtocolError::BufferTooShort);
    }
}
