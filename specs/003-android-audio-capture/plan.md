# Implementation Plan: Android Audio Capture & Mobile Client

**Branch**: `003-android-audio-capture` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/003-android-audio-capture/spec.md`

## Summary

Implementar el cliente móvil nativo para Android de PocketMic (Hito 1 y 2 móvil). Incluye un motor de captura de audio de baja latencia en C++ utilizando la biblioteca Google Oboe (AAudio / OpenSL ES) y codificación Opus embebida, enlace por sockets UDP con soporte de auto-descubrimiento en red local y fallback manual por IP, selector de cápsula física (inferior vs superior/frontal), sidetone local inmediato en auriculares y una interfaz moderna en Jetpack Compose con VU Meter en tiempo real y panel Stream Deck para disparar los efectos de voz en la PC.

## Technical Context

**Language/Version**: Kotlin 2.0+ (UI & Servicios Android), C++20 (Motor de audio Oboe / NDK r26+).

**Primary Dependencies**:
* Google Oboe (`com.google.oboe:oboe:1.9+`): Captura acústica AAudio / OpenSL ES de ultra-baja latencia.
* `libopus` (C estático compilado con CMake para `arm64-v8a` y `armeabi-v7a`).
* Jetpack Compose (`androidx.compose.*`): Interfaz de usuario reactiva y controles Stream Deck.
* Kotlin Coroutines & Flow: Telemetría y estado de red reactivo.

**Storage**: `DataStore` para almacenar la última dirección IP conectada y preferencias de cápsula.

**Testing**:
* Pruebas unitarias en C++ para el framer UDP y encoder Opus nativo.
* Pruebas de integración de captura en dispositivo físico Android / Emulador.

**Target Platform**: Android 8.0+ (API nivel 26 o superior).

**Project Type**: Android Application (Kotlin + C++ NDK).

**Performance Goals**: Latencia de captura interna $\le 6$ ms; consumo de batería $\le 5\%$ por hora; latencia de sidetone local $\le 2$ ms.

**Constraints**:
* Cumplimiento del Principio I de la Constitución: Cero llamadas que bloqueen el callback de audio nativo de Oboe.
* Ejecución obligatoria como `ForegroundService` con `foregroundServiceType="microphone"`.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Estado | Justificación de Cumplimiento |
| :--- | :---: | :--- |
| **I. Real-Time Determinism (Lock-Free)** | **PASS** | El callback `onAudioReady` de Oboe procesa muestras directamente en C++ sin pasar por el GC de Java ni realizar `malloc` dinámicos. |
| **II. Latency Budget & Zero Bloat** | **PASS** | Uso exclusivo de AAudio nativo de hardware. Latencia de adquisición $< 6$ ms. Cero frameworks pesados híbridos (React Native/Flutter descartados). |
| **III. Spec-Driven & Test-First** | **PASS** | Contratos JNI y protocolo binario de red estrictamente definidos antes de programar. |
| **IV. High Acoustic Fidelity (Float32)** | **PASS** | Adquisición a 48 kHz / 32-bit Float desde el conversor analógico-digital del smartphone. |
| **V. Resilient Connectivity** | **PASS** | Auto-descubrimiento en puerto 37718 con fallback para ingreso manual de IP. |

---

## Project Structure

### Documentation (this feature)

```text
specs/003-android-audio-capture/
├── spec.md              # Especificación funcional validada
├── plan.md              # Este plan de implementación
├── research.md          # Investigación técnica (Google Oboe, libopus NDK, MicrophoneDirection)
├── data-model.md        # Estructuras de datos (MicCapsule, MobileCaptureConfig)
├── quickstart.md        # Guía de ejecución y validación en smartphone
├── contracts/
│   └── mobile-native-bridge.md # Contrato JNI entre Kotlin y C++ Oboe
└── checklists/
    └── requirements.md  # Checklist de calidad de requisitos
```

### Source Code (repository root)

```text
mobile-android/
├── app/
│   ├── build.gradle.kts
│   └── src/
│       └── main/
│           ├── AndroidManifest.xml
│           ├── cpp/
│           │   ├── CMakeLists.txt
│           │   ├── audio_engine.cpp     # Motor Oboe (AAudio/OpenSL ES)
│           │   ├── audio_engine.h
│           │   ├── opus_encoder.cpp     # Wrapper nativo libopus
│           │   └── jni_bridge.cpp       # Enlace JNI con Kotlin
│           ├── java/io/pocketmic/
│           │   ├── core/
│           │   │   ├── AudioEngineBridge.kt
│           │   │   └── PocketMicService.kt # Foreground service de micrófono
│           │   ├── ui/
│           │   │   ├── MainActivity.kt
│           │   │   ├── components/
│           │   │   │   ├── VuMeter.kt     # Barra reactiva de dBFS con clipping
│           │   │   │   └── StreamDeckPad.kt # Cuadrícula de botones táctiles
│           │   │   └── theme/
│           │   └── net/
│           │       └── DiscoveryClient.kt # Broadcast UDP de búsqueda de PC
```

**Structure Decision**: El proyecto de Android reside limpiamente en `mobile-android/`, desacoplado del host de PC y estructurado con Kotlin nativo y C++ NDK.

---

## Complexity Tracking

*No violations detected.*
