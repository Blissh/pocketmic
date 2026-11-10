import numpy as np

class VoiceEffectType:
    BYPASS = 0
    AUTOTUNE = 1
    ANIME = 2
    CHIPMUNK = 3
    DEMON = 4
    TACTICAL_RADIO = 5
    ROBOT = 6
    MEGAPHONE = 7
    CATHEDRAL = 8
    BLEEP = 9

class MusicalScale:
    CHROMATIC = 0
    MAJOR = 1
    NATURAL_MINOR = 2
    MINOR_PENTATONIC = 3

class FxEngine:
    def __init__(self, sample_rate: int = 48000):
        self.sample_rate = sample_rate
        self.active_effect = VoiceEffectType.BYPASS
        self.bleep_active = False

        # Oscillators & phase
        self.bleep_phase = 0.0
        self.robot_phase = 0.0
        self.radio_x1 = 0.0
        self.radio_y1 = 0.0
        self.roger_beep_remaining = 0
        self.roger_beep_phase = 0.0

        # Anime formant state
        self.anime_x1 = 0.0

        # Autotune scale
        self.scale = MusicalScale.MINOR_PENTATONIC
        self.root_note = 9  # A

    def set_effect(self, effect_id: int):
        self.active_effect = effect_id

    def set_bleep(self, active: bool):
        self.bleep_active = active

    def quantize_pitch(self, freq: float) -> float:
        if freq < 60.0 or freq > 1200.0:
            return freq
        exact_midi = 69.0 + 12.0 * np.log2(freq / 440.0)
        rounded_midi = int(round(exact_midi))
        offset = (rounded_midi - self.root_note) % 12

        if self.scale == MusicalScale.MINOR_PENTATONIC:
            penta = [0, 3, 5, 7, 10]
            closest_degree = min(penta, key=lambda d: abs(d - offset))
        elif self.scale == MusicalScale.MAJOR:
            major = [0, 2, 4, 5, 7, 9, 11]
            closest_degree = min(major, key=lambda d: abs(d - offset))
        else:
            closest_degree = offset

        target_midi = (rounded_midi - offset) + closest_degree
        return 440.0 * (2.0 ** ((target_midi - 69) / 12.0))

    def process(self, samples: np.ndarray) -> np.ndarray:
        if self.bleep_active or self.active_effect == VoiceEffectType.BLEEP:
            return self._process_bleep(len(samples))

        if self.active_effect == VoiceEffectType.AUTOTUNE:
            return self._process_autotune(samples)
        elif self.active_effect == VoiceEffectType.ANIME:
            return self._process_anime(samples)
        elif self.active_effect == VoiceEffectType.TACTICAL_RADIO:
            return self._process_radio(samples)
        elif self.active_effect == VoiceEffectType.ROBOT:
            return self._process_robot(samples)
        elif self.active_effect == VoiceEffectType.DEMON:
            return self._process_demon(samples)
        elif self.active_effect == VoiceEffectType.CATHEDRAL:
            return self._process_reverb(samples)

        return samples

    def _process_bleep(self, length: int) -> np.ndarray:
        inc = 2.0 * np.pi * 1000.0 / self.sample_rate
        t = np.arange(length)
        out = 0.5 * np.sin(self.bleep_phase + t * inc).astype(np.float32)
        self.bleep_phase = (self.bleep_phase + length * inc) % (2.0 * np.pi)
        return out

    def _process_autotune(self, samples: np.ndarray) -> np.ndarray:
        # Zero-crossing estimation
        zc = np.where(np.diff(np.signbit(samples)))[0]
        if len(zc) > 2:
            est_freq = (len(zc) * self.sample_rate) / (2.0 * len(samples))
            if 70.0 < est_freq < 800.0:
                target_f = self.quantize_pitch(est_freq)
                ratio = target_f / est_freq
                # Resynthesis phase modulation
                step = (ratio - 1.0) * 0.4
                t = np.arange(len(samples))
                return (samples * (1.0 + step * np.sin(t * 0.1))).astype(np.float32)
        return samples

    def _process_anime(self, samples: np.ndarray) -> np.ndarray:
        # Pitch shift + Formant scaling
        # Shift spectral energy towards 2.5k - 4.5k Hz + non-linear harmonics
        out = np.empty_like(samples)
        for i, s in enumerate(samples):
            shaped = 1.18 * s - 0.82 * self.anime_x1
            self.anime_x1 = s
            out[i] = np.tanh(shaped * 1.33) * 0.85
        return out.astype(np.float32)

    def _process_radio(self, samples: np.ndarray) -> np.ndarray:
        out = np.empty_like(samples)
        for i, s in enumerate(samples):
            filt = 0.65 * (s - self.radio_x1) + 0.35 * self.radio_y1
            self.radio_x1 = s
            self.radio_y1 = filt
            out[i] = np.tanh(filt * 1.8) * 0.8
        return out.astype(np.float32)

    def _process_robot(self, samples: np.ndarray) -> np.ndarray:
        inc = 2.0 * np.pi * 75.0 / self.sample_rate
        t = np.arange(len(samples))
        carrier = np.sin(self.robot_phase + t * inc).astype(np.float32)
        self.robot_phase = (self.robot_phase + len(samples) * inc) % (2.0 * np.pi)
        return (0.4 * samples + 0.6 * (samples * carrier)).astype(np.float32)

    def _process_demon(self, samples: np.ndarray) -> np.ndarray:
        sub = np.sin(samples * 0.5).astype(np.float32)
        return (0.7 * samples + 0.4 * sub).astype(np.float32)

    def _process_reverb(self, samples: np.ndarray) -> np.ndarray:
        out = samples.copy()
        decay = 0.55
        for i in range(1, len(out)):
            out[i] += decay * out[i - 1]
        return (0.6 * samples + 0.4 * out).astype(np.float32)
