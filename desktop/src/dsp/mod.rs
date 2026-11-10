pub mod autotune;
pub mod formant_shifter;
pub mod meme_fx;
pub mod censor_beep;
pub mod fx_engine;

use std::convert::TryInto;

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
    Unknown(u8),
}

impl From<u8> for VoiceEffectType {
    fn from(val: u8) -> Self {
        match val {
            0x00 => VoiceEffectType::Bypass,
            0x01 => VoiceEffectType::Autotune,
            0x02 => VoiceEffectType::Anime,
            0x03 => VoiceEffectType::Chipmunk,
            0x04 => VoiceEffectType::Demon,
            0x05 => VoiceEffectType::TacticalRadio,
            0x06 => VoiceEffectType::Robot,
            0x07 => VoiceEffectType::Megaphone,
            0x08 => VoiceEffectType::CathedralReverb,
            0x09 => VoiceEffectType::CensorBeep,
            other => VoiceEffectType::Unknown(other),
        }
    }
}

impl From<VoiceEffectType> for u8 {
    fn from(fx: VoiceEffectType) -> Self {
        match fx {
            VoiceEffectType::Bypass => 0x00,
            VoiceEffectType::Autotune => 0x01,
            VoiceEffectType::Anime => 0x02,
            VoiceEffectType::Chipmunk => 0x03,
            VoiceEffectType::Demon => 0x04,
            VoiceEffectType::TacticalRadio => 0x05,
            VoiceEffectType::Robot => 0x06,
            VoiceEffectType::Megaphone => 0x07,
            VoiceEffectType::CathedralReverb => 0x08,
            VoiceEffectType::CensorBeep => 0x09,
            VoiceEffectType::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MusicalScale {
    Chromatic = 0,
    Major = 1,
    NaturalMinor = 2,
    MinorPentatonic = 3,
}

impl From<u8> for MusicalScale {
    fn from(val: u8) -> Self {
        match val {
            1 => MusicalScale::Major,
            2 => MusicalScale::NaturalMinor,
            3 => MusicalScale::MinorPentatonic,
            _ => MusicalScale::Chromatic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FxControlPacket {
    pub magic: u16,
    pub version: u8,
    pub packet_type: u8,
    pub effect_id: VoiceEffectType,
    pub mode: u8,
    pub param1: u16,
}

impl FxControlPacket {
    pub const SIZE: usize = 8;
    pub const MAGIC: u16 = 0x504D;
    pub const PACKET_TYPE_FX: u8 = 0x10;

    pub fn new(effect_id: VoiceEffectType, mode: u8, param1: u16) -> Self {
        Self {
            magic: Self::MAGIC,
            version: 1,
            packet_type: Self::PACKET_TYPE_FX,
            effect_id,
            mode,
            param1,
        }
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<(), &'static str> {
        if out.len() < Self::SIZE {
            return Err("Buffer too short");
        }
        out[0..2].copy_from_slice(&self.magic.to_be_bytes());
        out[2] = self.version;
        out[3] = self.packet_type;
        out[4] = self.effect_id.into();
        out[5] = self.mode;
        out[6..8].copy_from_slice(&self.param1.to_be_bytes());
        Ok(())
    }

    pub fn decode(buf: &[u8]) -> Result<Self, &'static str> {
        if buf.len() < Self::SIZE {
            return Err("Buffer too short");
        }
        let magic = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        if magic != Self::MAGIC {
            return Err("Invalid magic");
        }
        let version = buf[2];
        let packet_type = buf[3];
        let effect_id = VoiceEffectType::from(buf[4]);
        let mode = buf[5];
        let param1 = u16::from_be_bytes(buf[6..8].try_into().unwrap());

        Ok(Self {
            magic,
            version,
            packet_type,
            effect_id,
            mode,
            param1,
        })
    }
}
