#[cfg(test)]
mod tests {
    use pocketmic_daemon::audio::jitter_buffer::*;

    #[test]
    fn test_jitter_buffer_push_pop() {
        let (mut prod, mut cons) = JitterBuffer::new(1024);
        let samples = [0.1f32, 0.2, 0.3, 0.4];

        let written = prod.push_slice(&samples);
        assert_eq!(written, 4);
        assert_eq!(cons.occupied_len(), 4);

        let mut dest = [0.0f32; 4];
        let read = cons.pop_or_silence(&mut dest);
        assert_eq!(read, 4);
        assert_eq!(dest, samples);
    }

    #[test]
    fn test_jitter_buffer_underrun_silence_fill() {
        let (mut prod, mut cons) = JitterBuffer::new(1024);
        let samples = [0.5f32, 0.6];
        prod.push_slice(&samples);

        let mut dest = [1.0f32; 5];
        let read = cons.pop_or_silence(&mut dest);
        assert_eq!(read, 2);
        assert_eq!(dest[0], 0.5);
        assert_eq!(dest[1], 0.6);
        // The remaining 3 slots must be filled with silence (0.0)
        assert_eq!(dest[2], 0.0);
        assert_eq!(dest[3], 0.0);
        assert_eq!(dest[4], 0.0);
    }
}
