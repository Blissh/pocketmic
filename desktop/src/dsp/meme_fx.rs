pub struct MemeFx {
    robot_phase: f32,
    robot_phase_inc: f32,
    radio_x1: f32,
    radio_y1: f32,
    roger_beep_samples_left: usize,
    roger_beep_phase: f32,
}

impl MemeFx {
    pub const SAMPLE_RATE: f32 = 48000.0;

    pub fn new() -> Self {
        Self {
            robot_phase: 0.0,
            robot_phase_inc: 2.0 * std::f32::consts::PI * 75.0 / Self::SAMPLE_RATE,
            radio_x1: 0.0,
            radio_y1: 0.0,
            roger_beep_samples_left: 0,
            roger_beep_phase: 0.0,
        }
    }

    pub fn trigger_roger_beep(&mut self) {
        // 120 ms Roger beep = 5760 samples
        self.roger_beep_samples_left = (Self::SAMPLE_RATE * 0.12) as usize;
    }

    pub fn process_radio(&mut self, buffer: &mut [f32]) {
        // Bandpass + saturation
        for sample in buffer.iter_mut() {
            // High-pass + low-pass filter
            let filtered = 0.7 * (*sample - self.radio_x1) + 0.3 * self.radio_y1;
            self.radio_x1 = *sample;
            self.radio_y1 = filtered;

            // Soft-clipping saturation
            *sample = (filtered * 1.8).tanh() * 0.8;
        }

        // Overlay Roger Beep if active
        if self.roger_beep_samples_left > 0 {
            let freq = if self.roger_beep_samples_left > 2880 { 1000.0 } else { 1200.0 };
            let inc = 2.0 * std::f32::consts::PI * freq / Self::SAMPLE_RATE;
            for sample in buffer.iter_mut() {
                if self.roger_beep_samples_left == 0 {
                    break;
                }
                *sample += 0.35 * self.roger_beep_phase.sin();
                self.roger_beep_phase += inc;
                self.roger_beep_samples_left -= 1;
            }
        }
    }

    pub fn process_robot(&mut self, buffer: &mut [f32]) {
        // Ring modulation: sample * sin(75Hz)
        for sample in buffer.iter_mut() {
            let carrier = self.robot_phase.sin();
            self.robot_phase += self.robot_phase_inc;
            if self.robot_phase >= 2.0 * std::f32::consts::PI {
                self.robot_phase -= 2.0 * std::f32::consts::PI;
            }
            // 60% wet ring modulation
            *sample = 0.4 * *sample + 0.6 * (*sample * carrier);
        }
    }

    pub fn process_demon(&mut self, buffer: &mut [f32]) {
        // Deep voice pitch shift down + sub-octave distortion
        for sample in buffer.iter_mut() {
            let sub = (*sample * 0.5).sin();
            *sample = 0.7 * *sample + 0.4 * sub;
        }
    }

    pub fn process_reverb(&mut self, buffer: &mut [f32]) {
        // Cathedral Reverb simulation
        let decay = 0.65;
        let mut prev = 0.0f32;
        for sample in buffer.iter_mut() {
            let out = *sample + decay * prev;
            prev = out;
            *sample = 0.6 * *sample + 0.4 * out;
        }
    }
}
