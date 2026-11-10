#[cfg(test)]
mod tests {
    use pocketmic_daemon::audio::decoder::*;

    #[test]
    fn test_decoder_initialization() {
        let dec_mono = AudioDecoder::new(1).expect("Mono decoder should initialize");
        assert_eq!(dec_mono.channels(), 1);

        let dec_stereo = AudioDecoder::new(2).expect("Stereo decoder should initialize");
        assert_eq!(dec_stereo.channels(), 2);

        let err = AudioDecoder::new(3).expect_err("3 channels should fail");
        assert!(matches!(err, DecoderError::InvalidChannels(3)));
    }

    #[test]
    fn test_decoder_plc_concealment() {
        let mut dec = AudioDecoder::new(1).unwrap();
        let mut out = [1.0f32; 480];

        let samples = dec.decode_float(None, &mut out).expect("PLC decode should succeed");
        assert_eq!(samples, 480);
        // PLC output must be valid floating point numbers (zero/comfort)
        for s in out.iter() {
            assert_eq!(*s, 0.0);
        }
    }
}
