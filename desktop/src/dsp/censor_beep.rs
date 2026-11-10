pub struct CensorBeep {
    phase: f32,
    phase_increment: f32,
}

impl CensorBeep {
    pub const FREQUENCY_HZ: f32 = 1000.0;
    pub const SAMPLE_RATE: f32 = 48000.0;

    pub fn new() -> Self {
        Self {
            phase: 0.0,
            phase_increment: 2.0 * std::f32::consts::PI * Self::FREQUENCY_HZ / Self::SAMPLE_RATE,
        }
    }

    pub fn process_replace(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = 0.5 * self.phase.sin();
            self.phase += self.phase_increment;
            if self.phase >= 2.0 * std::f32::consts::PI {
                self.phase -= 2.0 * std::f32::consts::PI;
            }
        }
    }
}
