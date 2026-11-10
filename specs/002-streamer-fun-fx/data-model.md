# Data Model: Streamer Fun FX & Voice Changer

**Feature**: `002-streamer-fun-fx`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Entidades del Núcleo de Efectos

### 1.1. `VoiceEffectType` (Identificador de Preset)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VoiceEffectType {
    Bypass = 0x00,
    Autotune = 0x01,
    Anime = 0x02,
    Chipmunk = 0x03,
    Demon = 0x04,
    TacticalRadio = 0x05,
    Robot = 0x06,
    Megaphone = 0x07,
    CathedralReverb = 0x08,
    CensorBeep = 0x09,
}
```

---

### 1.2. `MusicalScale` (Escalas de Afinación para Autotune)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MusicalScale {
    Chromatic = 0,       // 12 semitonos
    Major = 1,           // Escala Mayor Natural (Jónica)
    NaturalMinor = 2,    // Escala Menor Natural (Eólica)
    MinorPentatonic = 3, // Pentatónica Menor (Sonido Trap / R&B clásico)
}

pub struct AutotuneConfig {
    pub scale: MusicalScale,
    pub root_note: u8,       // 0 = C, 1 = C#, 2 = D... 11 = B
    pub retune_speed_ms: u8, // 0 = Hard-Tune instantáneo
}
```

---

### 1.3. `AnimeVoiceConfig` (Parámetros de Voz Anime)
```rust
pub struct AnimeVoiceConfig {
    pub pitch_semitones: f32, // Rango +4.0 a +6.0 (defecto: +5.0)
    pub formant_scale: f32,   // Rango 1.15x a 1.25x (defecto: 1.20x)
    pub mix: f32,             // 1.0 (100% wet)
}
```

---

### 1.4. `RadioEffectConfig` (Parámetros de Radio Táctica)
```rust
pub struct RadioEffectConfig {
    pub low_cutoff_hz: f32,   // 350.0 Hz
    pub high_cutoff_hz: f32,  // 2800.0 Hz
    pub drive: f32,           // Factor de distorsión tanh (1.8x)
    pub roger_beep: bool,     // Emite doble tono al soltar
}
```

---

### 1.5. `FxControlPacket` (Comando Binario de Red)
Mensaje binario de 8 bytes enviado por el móvil a la PC:

```rust
#[repr(C, packed)]
pub struct FxControlPacket {
    pub magic: u16,      // 0x504D ("PM")
    pub version: u8,     // 0x01
    pub packet_type: u8, // 0x10 (FX_CMD)
    pub effect_id: u8,   // VoiceEffectType
    pub mode: u8,        // 0 = Toggle Off, 1 = Toggle On, 2 = Momentary (Push)
    pub param1: u16,     // Parámetro dependiente del efecto (Big-Endian)
}
```

* **Validaciones:**
  * `magic` DEBE ser `0x504D`.
  * `effect_id` debe mapearse a un `VoiceEffectType` válido ($0\le id \le 9$).
