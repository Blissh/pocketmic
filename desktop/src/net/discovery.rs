use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};

pub const DISCOVERY_PORT: u16 = 37718;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoverReq {
    pub cmd: String,
    pub client_id: String,
    pub device_name: String,
    pub protocol_version: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoverAck {
    pub cmd: String,
    pub server_name: String,
    pub audio_port: u16,
    pub protocol_version: u8,
    pub supported_codecs: Vec<String>,
    pub sample_rate: u32,
}

pub struct DiscoveryServer {
    socket: UdpSocket,
    server_name: String,
    audio_port: u16,
}

impl DiscoveryServer {
    pub fn bind(server_name: &str, audio_port: u16) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT))?;
        socket.set_nonblocking(true)?;
        Ok(Self {
            socket,
            server_name: server_name.to_string(),
            audio_port,
        })
    }

    pub fn poll_and_respond(&self) -> std::io::Result<Option<SocketAddr>> {
        let mut buf = [0u8; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                if let Ok(req) = serde_json::from_slice::<DiscoverReq>(&buf[..len]) {
                    if req.cmd == "DISCOVER_REQ" {
                        let ack = DiscoverAck {
                            cmd: "DISCOVER_ACK".to_string(),
                            server_name: self.server_name.clone(),
                            audio_port: self.audio_port,
                            protocol_version: 1,
                            supported_codecs: vec!["opus".to_string()],
                            sample_rate: 48000,
                        };
                        let resp_bytes = serde_json::to_vec(&ack).unwrap();
                        let _ = self.socket.send_to(&resp_bytes, src);
                        return Ok(Some(src));
                    }
                }
                Ok(None)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
