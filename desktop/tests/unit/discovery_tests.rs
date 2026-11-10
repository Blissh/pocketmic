#[cfg(test)]
mod tests {
    use pocketmic_daemon::net::discovery::*;

    #[test]
    fn test_discovery_json_serialization() {
        let req = DiscoverReq {
            cmd: "DISCOVER_REQ".to_string(),
            client_id: "test-client-123".to_string(),
            device_name: "Pixel 9 Pro".to_string(),
            protocol_version: 1,
        };

        let json = serde_json::to_string(&req).expect("Serialization should succeed");
        let decoded: DiscoverReq = serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(req, decoded);

        let ack = DiscoverAck {
            cmd: "DISCOVER_ACK".to_string(),
            server_name: "Gaming-PC".to_string(),
            audio_port: 37719,
            protocol_version: 1,
            supported_codecs: vec!["opus".to_string()],
            sample_rate: 48000,
        };

        let json_ack = serde_json::to_string(&ack).expect("Serialization should succeed");
        let decoded_ack: DiscoverAck = serde_json::from_str(&json_ack).expect("Deserialization should succeed");
        assert_eq!(ack, decoded_ack);
        assert_eq!(decoded_ack.audio_port, 37719);
    }
}
