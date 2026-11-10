#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected,
    Discovering,
    Handshaking,
    Streaming,
    Muted,
    Reconnecting,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkTelemetry {
    pub packets_received: u64,
    pub packets_lost: u64,
    pub last_sequence: u32,
    pub jitter_ms: f32,
    pub rtt_ms: f32,
}

impl NetworkTelemetry {
    pub fn record_packet(&mut self, seq: u32, _ts_us: u32) {
        if self.packets_received > 0 {
            if seq > self.last_sequence + 1 {
                let lost = (seq - self.last_sequence - 1) as u64;
                self.packets_lost += lost;
            }
        }
        self.packets_received += 1;
        self.last_sequence = seq;
    }

    pub fn packet_loss_rate(&self) -> f32 {
        let total = self.packets_received + self.packets_lost;
        if total == 0 {
            0.0
        } else {
            (self.packets_lost as f32 / total as f32) * 100.0
        }
    }
}
