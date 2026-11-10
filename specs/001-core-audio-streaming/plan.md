# Implementation Plan: Core Audio Streaming

**Branch**: `001-core-audio-streaming` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-core-audio-streaming/spec.md`

## Summary

Implementar el núcleo de transporte y reproducción de audio de PocketMic (Hito 1 - MVP). Consiste en un daemon de escritorio ligero en Rust que escucha datagramas de audio UDP (cabecera binaria `0x504D` de 16 bytes), desacopla el flujo mediante una cola circular lock-free (*SPSC Jitter Buffer*), decodifica frames Opus a 48 kHz / 32-bit Float y los reproduce en el subsistema de audio de Windows (WASAPI vía `cpal`) y Linux (PipeWire), acompañado de un servicio de auto-descubrimiento en red local y un harness de prueba sintética para validación end-to-end.

## Technical Context

**Language/Version**: Rust 1.80+ (PC Host), Python 3.10+ (Test harness sintético para validación end-to-end).

**Primary Dependencies**:
* `cpal` (v0.15+): Abstracción nativa de audio de ultra-baja latencia (WASAPI en Windows, PipeWire en Linux).
* `audiopus` / `opus`: Enlace nativo con `libopus` para decodificación con baja latencia y PLC.
* `ringbuf` (v0.4+): Colas circulares lock-free SPSC para transferir muestras entre el hilo de red y el callback de audio.
* `socket2`: Configuración avanzada de sockets UDP (`SO_RCVBUF`, non-blocking, broadcast).
* `serde`, `serde_json`: Manejo de mensajes de descubrimiento y handshake.

**Storage**: N/A (Totalmente en memoria volátil de tiempo real).

**Testing**:
* `cargo test`: Pruebas unitarias de serialización/deserialización de cabeceras, reordenamiento de paquetes y ring buffer.
* `tests/harness/mock_streamer.py`: Emisor sintético en Python capaz de inyectar tonos senoidales, ruido blanco o captura de micrófono local para validación cruzada.

**Target Platform**: Windows 10/11 (WASAPI), Linux (PipeWire).

**Project Type**: Desktop daemon / Audio service.

**Performance Goals**: Latencia end-to-end $\le 12$ ms (Wi-Fi 5 GHz) / $\le 5$ ms (USB); CPU $\le 2.0\%$ en PC; huella de RAM $\le 45$ MB.

**Constraints**:
* Cumplimiento estricto del Principio I de la Constitución: Cero `malloc`, cero llamadas bloqueantes y cero mutex en el callback de audio.
* Precisión de coma flotante de 32 bits (`f32`) a 48 kHz.

**Scale/Scope**: 1 flujo de audio bidireccional continuo con telemetría en tiempo real.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Estado | Justificación de Cumplimiento |
| :--- | :---: | :--- |
| **I. Real-Time Determinism (Lock-Free)** | **PASS** | El hilo de reproducción de audio solo consume de un `ringbuf::Consumer` pre-asignado. Cero asignaciones o bloqueos en el callback. |
| **II. Latency Budget & Zero Bloat** | **PASS** | Frames Opus de 10 ms (480 muestras) + Jitter buffer elástico de 2 a 4 frames (latencia acumulada $< 15$ ms). Binario nativo de Rust $< 15$ MB. |
| **III. Spec-Driven & Test-First** | **PASS** | Todas las estructuras binarias y secuencias de red están formalizadas en `contracts/` y validadas con suite de pruebas unitarias y harness sintético. |
| **IV. High Acoustic Fidelity (Float32)** | **PASS** | Decodificación directa de Opus a muestras `f32` a 48.000 Hz, entregadas directamente al renderer WASAPI. |
| **V. Resilient Connectivity** | **PASS** | Protocolo UDP con números de secuencia monótonos para detección de pérdidas, soporte de PLC de Opus y puerto de descubrimiento broadcast + soporte de IP manual. |

---

## Project Structure

### Documentation (this feature)

```text
specs/001-core-audio-streaming/
├── spec.md              # Especificación funcional validada
├── plan.md              # Este plan de implementación
├── research.md          # Investigación de Phase 0 (cpal, opus, ringbuf)
├── data-model.md        # Estructuras de datos (AudioPacketHeader, AudioFrame, SessionState)
├── quickstart.md        # Guía paso a paso de verificación y ejecución
├── contracts/
│   └── wire-protocol.md # Especificación exacta del protocolo binario UDP
└── checklists/
    └── requirements.md  # Checklist de calidad de requisitos
```

### Source Code (repository root)

```text
desktop/
├── Cargo.toml
├── src/
│   ├── main.rs                   # Punto de entrada CLI y arranque del daemon
│   ├── session.rs                # Máquina de estados y telemetría de red
│   ├── net/
│   │   ├── mod.rs
│   │   ├── protocol.rs           # Deserializador binario de cabecera (16 bytes)
│   │   ├── udp_receiver.rs       # Hilo receptor de datagramas de audio
│   │   └── discovery.rs          # Servidor UDP broadcast para auto-descubrimiento
│   └── audio/
│       ├── mod.rs
│       ├── decoder.rs            # Decodificador Opus (f32 @ 48kHz con PLC)
│       ├── engine.rs             # Driver de salida WASAPI / PipeWire (CPAL)
│       └── jitter_buffer.rs      # Cola circular lock-free SPSC (ringbuf)
│
tests/
└── harness/
    ├── mock_streamer.py          # Emisor de audio sintético UDP para pruebas
    └── discovery_probe.py        # Sonda de prueba de descubrimiento de red
```

**Structure Decision**: Arquitectura modular desacoplada: el directorio `desktop/` contiene el daemon de escritorio en Rust de alto rendimiento, mientras que `tests/harness/` contiene las herramientas de simulación y verificación sintética multiplataforma.

---

## Complexity Tracking

*No violations detected. Todo el diseño satisface estrictamente los principios constitucionales del proyecto.*
