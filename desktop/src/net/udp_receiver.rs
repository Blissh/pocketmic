use crate::audio::decoder::AudioDecoder;
use crate::audio::jitter_buffer::JitterBufferProducer;
use crate::net::protocol::{AudioPacketHeader, HEADER_SIZE};
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct UdpReceiver {
    socket: UdpSocket,
    is_running: Arc<AtomicBool>,
}

impl UdpReceiver {
    pub fn bind(addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;
        Ok(Self {
            socket,
            is_running: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.socket.local_addr()
    }

    /// Processes one incoming packet if available.
    /// Returns Ok(true) if a packet was processed, Ok(false) if WouldBlock, Err on I/O failure.
    pub fn poll_and_process(
        &self,
        buf: &mut [u8],
        decoder: &mut AudioDecoder,
        producer: &mut JitterBufferProducer,
        decoded_pcm: &mut [f32],
    ) -> std::io::Result<bool> {
        match self.socket.recv_from(buf) {
            Ok((len, _src)) => {
                if len < HEADER_SIZE {
                    return Ok(false); // Ignore runt packet
                }

                if let Ok(header) = AudioPacketHeader::decode(&buf[..HEADER_SIZE]) {
                    let payload_end = HEADER_SIZE + (header.payload_len as usize);
                    let payload = if len >= payload_end {
                        Some(&buf[HEADER_SIZE..payload_end])
                    } else {
                        None
                    };

                    // Decode and push to lock-free jitter buffer
                    if let Ok(samples_decoded) = decoder.decode_float(payload, decoded_pcm) {
                        if samples_decoded > 0 {
                            producer.push_slice(&decoded_pcm[..samples_decoded]);
                        }
                    }
                }
                Ok(true)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(false),
            Err(e) => Err(e),
        }
    }
}
