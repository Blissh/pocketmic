use crate::dsp::autotune::Autotune;
use crate::dsp::censor_beep::CensorBeep;
use crate::dsp::formant_shifter::FormantShifter;
use crate::dsp::meme_fx::MemeFx;
use crate::dsp::{MusicalScale, VoiceEffectType};

pub struct FxEngine {
    current_effect: VoiceEffectType,
    previous_effect: VoiceEffectType,
    crossfade_samples_left: usize,
    autotune: Autotune,
    anime: FormantShifter,
    meme: MemeFx,
    censor: CensorBeep,
    is_bleep_active: bool,
}

impl FxEngine {
    pub const CROSSFADE_LEN: usize = 96; // 2 ms @ 48 kHz

    pub fn new() -> Self {
        Self {
            current_effect: VoiceEffectType::Bypass,
            previous_effect: VoiceEffectType::Bypass,
            crossfade_samples_left: 0,
            autotune: Autotune::new(MusicalScale::MinorPentatonic, 9), // Default A Minor Pentatonic
            anime: FormantShifter::new(),
            meme: MemeFx::new(),
            censor: CensorBeep::new(),
            is_bleep_active: false,
        }
    }

    pub fn set_effect(&mut self, new_fx: VoiceEffectType) {
        if new_fx != self.current_effect {
            self.previous_effect = self.current_effect;
            self.current_effect = new_fx;
            self.crossfade_samples_left = Self::CROSSFADE_LEN;
        }
    }

    pub fn set_bleep(&mut self, active: bool) {
        self.is_bleep_active = active;
    }

    pub fn current_effect(&self) -> VoiceEffectType {
        self.current_effect
    }

    /// Process audio buffer in place (strictly lock-free and allocation-free)
    pub fn process(&mut self, buffer: &mut [f32]) {
        if self.is_bleep_active {
            self.censor.process_replace(buffer);
            return;
        }

        match self.current_effect {
            VoiceEffectType::Bypass => {}
            VoiceEffectType::Autotune => self.autotune.process(buffer),
            VoiceEffectType::Anime => self.anime.process(buffer),
            VoiceEffectType::Chipmunk => self.anime.process(buffer),
            VoiceEffectType::Demon => self.meme.process_demon(buffer),
            VoiceEffectType::TacticalRadio => self.meme.process_radio(buffer),
            VoiceEffectType::Robot => self.meme.process_robot(buffer),
            VoiceEffectType::CathedralReverb => self.meme.process_reverb(buffer),
            VoiceEffectType::CensorBeep => self.censor.process_replace(buffer),
            _ => {}
        }
    }
}
