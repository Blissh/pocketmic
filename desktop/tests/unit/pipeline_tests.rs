#[cfg(test)]
mod tests {
    use pocketmic_daemon::audio::decoder::*;
    use pocketmic_daemon::audio::jitter_buffer::*;
    use pocketmic_daemon::net::protocol::*;

    #[test]
    fn test_pipeline_packet_to_jitter_buffer() {
        let (mut prod, mut cons) = JitterBuffer::new(4800);
        let mut decoder = AudioDecoder::new(1).unwrap();

        // Create synthetic raw frame
        let mut test_samples = [0.0f32; 480];
        test_samples[0] = 0.42;
        let mut raw_bytes = Vec::with_capacity(480 * 4);
        for s in test_samples.iter() {
            raw_bytes.extend_from_slice(&s.to_le_bytes());
        }

        let header = AudioPacketHeader::new(
            PacketType::Audio,
            1,
            10000,
            0,
            1,
            raw_bytes.len() as u16,
        );

        // Decode
        let mut decoded = [0.0f32; 480];
        let n = decoder.decode_float(Some(&raw_bytes), &mut decoded).unwrap();
        assert_eq!(n, 480);

        // Push to lock-free jitter buffer
        let pushed = prod.push_slice(&decoded);
        assert_eq!(pushed, 480);

        // Consumer reads in audio callback
        let mut audio_callback_buf = [0.0f32; 480];
        let read = cons.pop_or_silence(&mut audio_callback_buf);
        assert_eq!(read, 480);
        assert_eq!(audio_callback_buf[0], 0.42);
    }
}
