#[cfg(test)]
mod tests {
    use pocketmic_daemon::dsp::fx_engine::*;
    use pocketmic_daemon::dsp::VoiceEffectType;

    #[test]
    fn test_fx_engine_switching() {
        let mut engine = FxEngine::new();
        assert_eq!(engine.current_effect(), VoiceEffectType::Bypass);

        engine.set_effect(VoiceEffectType::Autotune);
        assert_eq!(engine.current_effect(), VoiceEffectType::Autotune);

        let mut buffer = [0.1f32; 480];
        engine.process(&mut buffer);

        // Verify output is clean
        assert!(buffer[0].is_finite());
    }

    #[test]
    fn test_censor_bleep_override() {
        let mut engine = FxEngine::new();
        engine.set_effect(VoiceEffectType::Autotune);

        // Turn bleep on
        engine.set_bleep(true);
        let mut buffer = [0.0f32; 480];
        engine.process(&mut buffer);

        // Sine wave peak should be around 0.5
        let max_val = buffer.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        assert!(max_val > 0.45 && max_val <= 0.55);
    }
}
