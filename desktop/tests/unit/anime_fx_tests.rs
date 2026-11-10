#[cfg(test)]
mod tests {
    use pocketmic_daemon::dsp::formant_shifter::*;

    #[test]
    fn test_anime_formant_shifter_stability() {
        let mut shifter = FormantShifter::new();
        let mut buffer = [0.2f32; 480];

        shifter.process(&mut buffer);

        // Verify output is bounded and finite
        for &s in buffer.iter() {
            assert!(s.is_finite());
            assert!(s.abs() <= 1.0);
        }
    }
}
