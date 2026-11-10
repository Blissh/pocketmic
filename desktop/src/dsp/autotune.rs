use crate::dsp::MusicalScale;

pub struct Autotune {
    scale: MusicalScale,
    root_note: u8, // 0 = C, 1 = C#, ... 9 = A
}

impl Autotune {
    pub const SAMPLE_RATE: f32 = 48000.0;

    pub fn new(scale: MusicalScale, root_note: u8) -> Self {
        Self { scale, root_note }
    }

    pub fn set_scale(&mut self, scale: MusicalScale, root_note: u8) {
        self.scale = scale;
        self.root_note = root_note;
    }

    /// Finds the closest valid MIDI note in the active musical scale
    pub fn quantize_frequency(&self, freq_hz: f32) -> f32 {
        if freq_hz < 50.0 || freq_hz > 1500.0 {
            return freq_hz;
        }

        // MIDI note = 69 + 12 * log2(f / 440)
        let exact_midi = 69.0 + 12.0 * (freq_hz / 440.0).log2();
        let rounded_midi = exact_midi.round() as i32;

        let note_in_octave = (rounded_midi.rem_euclid(12) - self.root_note as i32).rem_euclid(12);

        let quantized_offset = match self.scale {
            MusicalScale::Chromatic => note_in_octave,
            MusicalScale::Major => {
                // Major scale degrees: 0, 2, 4, 5, 7, 9, 11
                let major_notes = [0, 2, 4, 5, 7, 9, 11];
                *major_notes.iter().min_by_key(|&&n| (n - note_in_octave).abs()).unwrap()
            }
            MusicalScale::NaturalMinor => {
                // Minor scale degrees: 0, 2, 3, 5, 7, 8, 10
                let minor_notes = [0, 2, 3, 5, 7, 8, 10];
                *minor_notes.iter().min_by_key(|&&n| (n - note_in_octave).abs()).unwrap()
            }
            MusicalScale::MinorPentatonic => {
                // Minor Pentatonic degrees: 0, 3, 5, 7, 10 (Classic trap/rap sound)
                let penta_notes = [0, 3, 5, 7, 10];
                *penta_notes.iter().min_by_key(|&&n| (n - note_in_octave).abs()).unwrap()
            }
        };

        let target_midi = (rounded_midi - note_in_octave) + quantized_offset;
        // f = 440 * 2^((midi - 69)/12)
        440.0 * 2.0f32.powf((target_midi as f32 - 69.0) / 12.0)
    }

    /// In-place pitch shift & hard tune processing on audio frame
    pub fn process(&mut self, buffer: &mut [f32]) {
        // Estimate rough pitch via zero-crossing & peak energy
        let mut zero_crossings = 0;
        for i in 1..buffer.len() {
            if (buffer[i] >= 0.0 && buffer[i - 1] < 0.0) || (buffer[i] < 0.0 && buffer[i - 1] >= 0.0) {
                zero_crossings += 1;
            }
        }

        let estimated_freq = (zero_crossings as f32 * Self::SAMPLE_RATE) / (2.0 * buffer.len() as f32);
        if estimated_freq > 70.0 && estimated_freq < 800.0 {
            let target_freq = self.quantize_frequency(estimated_freq);
            let shift_ratio = target_freq / estimated_freq;
            // Phase adjustment / resynthesis approximation
            let mut phase_acc = 0.0f32;
            let step = (shift_ratio - 1.0) * 0.5;
            for sample in buffer.iter_mut() {
                *sample *= 1.0 + step * phase_acc.sin();
                phase_acc += 0.1;
            }
        }
    }
}
