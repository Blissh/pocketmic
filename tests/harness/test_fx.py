import unittest
import numpy as np
from tests.harness.fx_engine import FxEngine, VoiceEffectType, MusicalScale
import struct

class TestFxEngine(unittest.TestCase):
    def setUp(self):
        self.engine = FxEngine(sample_rate=48000)

    def test_autotune_quantize(self):
        self.engine.scale = MusicalScale.MINOR_PENTATONIC
        self.engine.root_note = 9  # A
        # 440 Hz is exactly A4 -> should stay 440
        q1 = self.engine.quantize_pitch(440.0)
        self.assertAlmostEqual(q1, 440.0, delta=1.0)

        # 455 Hz is between A4 (440) and C5 (523.25) -> should quantize to 440 or 523.25
        q2 = self.engine.quantize_pitch(455.0)
        self.assertTrue(abs(q2 - 440.0) < 1.0 or abs(q2 - 523.25) < 1.0)

    def test_anime_voice_filter(self):
        self.engine.set_effect(VoiceEffectType.ANIME)
        sine = 0.5 * np.sin(2 * np.pi * 150.0 * np.arange(480) / 48000).astype(np.float32)
        out = self.engine.process(sine)
        self.assertEqual(len(out), 480)
        self.assertTrue(np.all(np.isfinite(out)))
        self.assertTrue(np.max(np.abs(out)) <= 1.0)

    def test_censor_bleep(self):
        self.engine.set_bleep(True)
        silence = np.zeros(480, dtype=np.float32)
        out = self.engine.process(silence)
        # Should generate 1000 Hz sine with peak 0.5
        self.assertAlmostEqual(float(np.max(out)), 0.5, delta=0.05)

    def test_radio_and_robot(self):
        self.engine.set_effect(VoiceEffectType.TACTICAL_RADIO)
        sine = 0.5 * np.sin(2 * np.pi * 440.0 * np.arange(480) / 48000).astype(np.float32)
        radio_out = self.engine.process(sine)
        self.assertTrue(np.all(np.isfinite(radio_out)))

        self.engine.set_effect(VoiceEffectType.ROBOT)
        robot_out = self.engine.process(sine)
        self.assertTrue(np.all(np.isfinite(robot_out)))

if __name__ == "__main__":
    unittest.main()
