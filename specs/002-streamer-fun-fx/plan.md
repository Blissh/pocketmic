# Implementation Plan: Streamer Fun FX & Voice Changer

**Branch**: `002-streamer-fun-fx` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/002-streamer-fun-fx/spec.md`

## Summary

Implementar el motor de procesamiento digital de señales (DSP) en tiempo real para efectos de streamer en PocketMic (Hito 4). Este módulo procesa los frames Float32 a 48 kHz inmediatamente después de la decodificación de audio, aplicando algoritmos de afinación percusiva (*Autotune* por cuantización de notas en escalas musicales), modulación de voz estilo Anime ("Monita China") mediante separación de pitch fundamental y escalado formántico, efectos meme (Radio Táctica con Roger Beep, Demonio, Robot Sci-Fi, Catedral Reverb) y un oscilador de censura instantáneo (*Push-to-Bleep* a 1.000 Hz) con fundidos cruzados de 2 ms sin clics digitales.

## Technical Context

**Language/Version**: Rust 1.80+ (PC Host), Python 3.10+ (Harness de referencia y pruebas sintéticas de audio).

**Primary Dependencies**:
* Algoritmos matemáticos nativos lock-free (filtros biquad IIR direct form II transposed, tablas trigonométricas pre-computadas).
* `ringbuf`: Buffers circulares para el delay del pitch-shifter y del Freeverb.
* `byteorder`: Deserialización del datagrama de control de 8 bytes (`FxControlPacket`).

**Storage**: N/A (Totalmente en memoria y buffers pre-asignados estáticos).

**Testing**:
* `desktop/tests/unit/fx_tests.rs`: Pruebas de cuantización matemática de notas, filtros de radio y crossfades.
* `tests/harness/test_fx.py`: Verificación de forma de onda, FFT de formantes y latencia algorítmica.

**Target Platform**: Windows 10/11 (WASAPI), Linux (PipeWire).

**Project Type**: DSP Audio Processing Engine.

**Performance Goals**: Latencia adicional por DSP $\le 5$ ms; CPU adicional $\le 1.5\%$; latencia de conmutación de efecto $< 1.5$ ms.

**Constraints**:
* Cumplimiento estricto del Principio I de la Constitución: Cero llamadas a `malloc`/`free` dentro del callback de procesamiento.
* Todo el procesamiento opera con precisión de coma flotante de 32 bits (`f32`) a 48.000 Hz.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Estado | Justificación de Cumplimiento |
| :--- | :---: | :--- |
| **I. Real-Time Determinism (Lock-Free)** | **PASS** | Todas las tablas de senos, estados de filtros biquad y buffers de retardo se inicializan estáticamente al instanciar el motor. Cero asignaciones en el callback de audio. |
| **II. Latency Budget & Zero Bloat** | **PASS** | Latencia algorítmica agregada $< 5$ ms. El cálculo de filtros IIR y multiplicación de formantes consume $< 1.2\%$ de CPU. |
| **III. Spec-Driven & Test-First** | **PASS** | Tablas de cuantización de notas, rangos de frecuencia y contratos de paquetes binarios especificados antes de codificar. |
| **IV. High Acoustic Fidelity (Float32)** | **PASS** | Todas las operaciones DSP (multiplicaciones de envolvente, saturación tanh, filtros) operan en `f32` a 48 kHz. |
| **V. Resilient Connectivity** | **PASS** | Comandos de control de 8 bytes enviados por UDP con tolerancia a pérdida y recuperación automática de estado. |

---

## Project Structure

### Documentation (this feature)

```text
specs/002-streamer-fun-fx/
├── spec.md              # Especificación funcional validada
├── plan.md              # Este plan de implementación
├── research.md          # Investigación técnica (YIN, Formants, Freeverb, Biquad)
├── data-model.md        # Estructuras de datos (VoiceEffectType, AutotuneConfig, FxControlPacket)
├── quickstart.md        # Guía paso a paso de verificación de efectos
├── contracts/
│   └── fx-control-protocol.md # Protocolo binario de control (8 bytes)
└── checklists/
    └── requirements.md  # Checklist de calidad de requisitos
```

### Source Code (repository root)

```text
desktop/
└── src/
    └── dsp/
        ├── mod.rs
        ├── fx_engine.rs         # Enrutador principal de efectos y mezclador de crossfade
        ├── autotune.rs          # Detector de pitch y cuantizador a notas musicales
        ├── formant_shifter.rs   # Motor vocal Anime (Pitch Shift + Formant Scaling)
        ├── meme_fx.rs           # Radio táctica (Biquad + Roger Beep), Demonio, Robot, Reverb
        └── censor_beep.rs       # Oscilador senoidal a 1 kHz con rampa de 2 ms
│
tests/
└── harness/
    ├── fx_engine.py             # Implementación de referencia de los efectos en Python
    ├── fx_probe.py              # CLI para enviar comandos de efectos en vivo por UDP
    └── test_fx.py               # Suite de pruebas unitarias y de análisis espectral
```

**Structure Decision**: El motor DSP se ubica modularmente bajo `desktop/src/dsp/`, integrándose entre el decodificador de audio y el motor de salida de WASAPI, con su contraparte de prueba en `tests/harness/`.

---

## Complexity Tracking

*No violations detected.*
