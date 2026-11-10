import struct
from typing import Tuple

MAGIC_ID = 0x504D  # 'PM'
PROTOCOL_VERSION = 1
HEADER_SIZE = 16

class PacketType:
    AUDIO = 1
    PING = 2
    PONG = 3

class HeaderFlags:
    MUTE = 0x01
    FEC = 0x02
    CLIPPING = 0x04

class AudioPacketHeader:
    def __init__(self, packet_type: int, sequence_number: int, timestamp_us: int, flags: int, channels: int, payload_len: int):
        self.magic = MAGIC_ID
        self.version = PROTOCOL_VERSION
        self.packet_type = packet_type
        self.sequence_number = sequence_number
        self.timestamp_us = timestamp_us
        self.flags = flags
        self.channels = channels
        self.payload_len = payload_len

    def encode(self) -> bytes:
        # > = Big-Endian: H (u16), B (u8), B (u8), I (u32), I (u32), B (u8), B (u8), H (u16)
        return struct.pack(
            ">HBBIIBBH",
            self.magic,
            self.version,
            self.packet_type,
            self.sequence_number,
            self.timestamp_us,
            self.flags,
            self.channels,
            self.payload_len
        )

    @classmethod
    def decode(cls, data: bytes) -> "AudioPacketHeader":
        if len(data) < HEADER_SIZE:
            raise ValueError(f"Buffer too short: {len(data)} < {HEADER_SIZE}")
        magic, version, packet_type, seq, ts, flags, channels, payload_len = struct.unpack(
            ">HBBIIBBH", data[:HEADER_SIZE]
        )
        if magic != MAGIC_ID:
            raise ValueError(f"Invalid magic: {hex(magic)}, expected {hex(MAGIC_ID)}")
        if version != PROTOCOL_VERSION:
            raise ValueError(f"Unsupported protocol version: {version}")
        return cls(packet_type, seq, ts, flags, channels, payload_len)
