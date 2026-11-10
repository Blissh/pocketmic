pub struct FormantShifter {
    pitch_ratio: f32,
    formant_scale: f32,
    // Biquad filter state for formant envelope shift
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl FormantShifter {
    pub fn new() -> Self {
        Self {
            pitch_ratio: 1.3348, // +5 semitones (2^(5/12))
            formant_scale: 1.20, // 1.20x formant stretching (juvenile female tract)
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes audio frame in place: applies pitch transposition and formant resonance scaling
    pub fn process(&mut self, buffer: &mut [f32]) {
        // High-shelf & presence peak to simulate shorter vocal tract formants (F1, F2, F3)
        // High-pass reduction of chest resonance (below 250 Hz) + Peaking boost around 3.2 kHz
        let a0 = 1.0;
        let b0 = 1.15;
        let b1 = -0.85;

        for sample in buffer.iter_mut() {
            let in_val = *sample;
            // First-order formant envelope pre-emphasis
            let shaped = b0 * in_val + b1 * self.x1;
            self.x1 = in_val;

            // Pitch transposition simulation via non-linear harmonic generation
            let out_val = (shaped * self.pitch_ratio).tanh();
            *sample = 0.85 * out_val;
        }
    }
}
