#[cfg(test)]
mod tests {
    use pocketmic_daemon::dsp::autotune::*;
    use pocketmic_daemon::dsp::MusicalScale;

    #[test]
    fn test_autotune_quantize_chromatic() {
        let at = Autotune::new(MusicalScale::Chromatic, 0);
        // A4 = 440 Hz
        let q = at.quantize_frequency(443.0);
        assert!((q - 440.0).abs() < 1.0);
    }

    #[test]
    fn test_autotune_quantize_pentatonic() {
        let at = Autotune::new(MusicalScale::MinorPentatonic, 9); // A Minor Pentatonic: A, C, D, E, G
        // 440 Hz (A4) is in the scale
        let q = at.quantize_frequency(440.0);
        assert!((q - 440.0).abs() < 1.0);

        // Near 466 Hz (A#4, not in A minor penta), should quantize to A4 (440) or C5 (523.25)
        let q2 = at.quantize_frequency(455.0);
        assert!(q2 == 440.0 || (q2 - 523.25).abs() < 1.0);
    }
}
