use std::convert::TryInto;

pub const MAGIC_ID: u16 = 0x504D; // ASCII 'P' 'M'
pub const PROTOCOL_VERSION: u8 = 0x01;
pub const HEADER_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Audio = 0x01,
    Ping = 0x02,
    Pong = 0x03,
    Unknown(u8),
}

impl From<u8> for PacketType {
    fn from(val: u8) -> Self {
        match val {
            0x01 => PacketType::Audio,
            0x02 => PacketType::Ping,
            0x03 => PacketType::Pong,
            other => PacketType::Unknown(other),
        }
    }
}

impl From<PacketType> for u8 {
    fn from(pt: PacketType) -> Self {
        match pt {
            PacketType::Audio => 0x01,
            PacketType::Ping => 0x02,
            PacketType::Pong => 0x03,
            PacketType::Unknown(val) => val,
        }
    }
}

pub struct HeaderFlags;

impl HeaderFlags {
    pub const MUTE: u8 = 0x01;
    pub const FEC: u8 = 0x02;
    pub const CLIPPING: u8 = 0x04;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioPacketHeader {
    pub magic: u16,
    pub version: u8,
    pub packet_type: PacketType,
    pub sequence_number: u32,
    pub timestamp_us: u32,
    pub flags: u8,
    pub channels: u8,
    pub payload_len: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
    BufferTooShort,
    InvalidMagic(u16),
    UnsupportedVersion(u8),
    PayloadLengthMismatch { expected: usize, actual: usize },
}

impl AudioPacketHeader {
    pub fn new(
        packet_type: PacketType,
        sequence_number: u32,
        timestamp_us: u32,
        flags: u8,
        channels: u8,
        payload_len: u16,
    ) -> Self {
        Self {
            magic: MAGIC_ID,
            version: PROTOCOL_VERSION,
            packet_type,
            sequence_number,
            timestamp_us,
            flags,
            channels,
            payload_len,
        }
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<(), ProtocolError> {
        if out.len() < HEADER_SIZE {
            return Err(ProtocolError::BufferTooShort);
        }
        out[0..2].copy_from_slice(&self.magic.to_be_bytes());
        out[2] = self.version;
        out[3] = self.packet_type.into();
        out[4..8].copy_from_slice(&self.sequence_number.to_be_bytes());
        out[8..12].copy_from_slice(&self.timestamp_us.to_be_bytes());
        out[12] = self.flags;
        out[13] = self.channels;
        out[14..16].copy_from_slice(&self.payload_len.to_be_bytes());
        Ok(())
    }

    pub fn decode(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < HEADER_SIZE {
            return Err(ProtocolError::BufferTooShort);
        }

        let magic = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        if magic != MAGIC_ID {
            return Err(ProtocolError::InvalidMagic(magic));
        }

        let version = buf[2];
        if version != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion(version));
        }

        let packet_type = PacketType::from(buf[3]);
        let sequence_number = u32::from_be_bytes(buf[4..8].try_into().unwrap());
        let timestamp_us = u32::from_be_bytes(buf[8..12].try_into().unwrap());
        let flags = buf[12];
        let channels = buf[13];
        let payload_len = u16::from_be_bytes(buf[14..16].try_into().unwrap());

        Ok(Self {
            magic,
            version,
            packet_type,
            sequence_number,
            timestamp_us,
            flags,
            channels,
            payload_len,
        })
    }

    pub fn is_muted(&self) -> bool {
        (self.flags & HeaderFlags::MUTE) != 0
    }

    pub fn has_fec(&self) -> bool {
        (self.flags & HeaderFlags::FEC) != 0
    }

    pub fn is_clipping(&self) -> bool {
        (self.flags & HeaderFlags::CLIPPING) != 0
    }
}
