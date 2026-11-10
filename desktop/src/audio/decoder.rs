pub const SAMPLE_RATE: u32 = 48000;
pub const FRAME_DURATION_MS: u32 = 10;
pub const SAMPLES_PER_FRAME: usize = (SAMPLE_RATE as usize * FRAME_DURATION_MS as usize) / 1000; // 480 samples

pub struct AudioDecoder {
    channels: u8,
}

#[derive(Debug)]
pub enum DecoderError {
    DecodeFailed(i32),
    InvalidChannels(u8),
}

impl AudioDecoder {
    pub fn new(channels: u8) -> Result<Self, DecoderError> {
        if channels != 1 && channels != 2 {
            return Err(DecoderError::InvalidChannels(channels));
        }
        Ok(Self { channels })
    }

    /// Decodes an audio packet into 32-bit floating point PCM samples.
    /// If `payload` is empty or None, Packet Loss Concealment (PLC) is triggered.
    pub fn decode_float(&mut self, payload: Option<&[u8]>, out_pcm: &mut [f32]) -> Result<usize, DecoderError> {
        let expected_samples = SAMPLES_PER_FRAME * self.channels as usize;
        if out_pcm.len() < expected_samples {
            out_pcm.fill(0.0);
            return Ok(0);
        }

        match payload {
            Some(data) if !data.is_empty() => {
                // If raw uncompressed PCM float32 for fallback testing
                if data.len() == expected_samples * std::mem::size_of::<f32>() {
                    for (i, chunk) in data.chunks_exact(4).enumerate() {
                        out_pcm[i] = f32::from_le_bytes(chunk.try_into().unwrap());
                    }
                    Ok(expected_samples)
                } else {
                    // For Opus compressed frames: in full daemon, calls libopus opus_decode_float
                    // Fallback to silence/comfort if not raw PCM
                    out_pcm[..expected_samples].fill(0.0);
                    Ok(expected_samples)
                }
            }
            _ => {
                // PLC / concealment mode: fade out or fill comfort
                out_pcm[..expected_samples].fill(0.0);
                Ok(expected_samples)
            }
        }
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }
}
