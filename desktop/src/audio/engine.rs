use crate::audio::jitter_buffer::JitterBufferConsumer;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

pub struct AudioOutputConfig {
    pub sample_rate: u32,
    pub channels: u16,
}

impl Default for AudioOutputConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            channels: 1,
        }
    }
}

pub struct AudioOutputEngine {
    config: AudioOutputConfig,
    is_running: Arc<AtomicBool>,
}

impl AudioOutputEngine {
    pub fn new(config: AudioOutputConfig) -> Self {
        Self {
            config,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Pulls samples from the jitter buffer and copies to output slice.
    /// Strictly lock-free and allocation-free for the audio callback thread.
    #[inline(always)]
    pub fn process_callback(consumer: &mut JitterBufferConsumer, output: &mut [f32]) {
        consumer.pop_or_silence(output);
    }
}
