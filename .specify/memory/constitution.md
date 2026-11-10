<!--
# Sync Impact Report
- Version change: none -> 1.0.0 (Initial Ratification)
- List of modified principles: Initial definition of 5 core principles
  * I. Real-Time Determinism & Lock-Free Audio Thread (NON-NEGOTIABLE)
  * II. Latency Budget & Zero Bloat (NON-NEGOTIABLE)
  * III. Spec-Driven & Test-First (TDD / SDD)
  * IV. High Acoustic Fidelity (Float32 @ 48 kHz)
  * V. Resilient Connectivity & Zero Friction
- Added sections:
  * Technology Stack & Hardware Boundary Constraints
  * Quality Gates & Performance Benchmarking
- Removed sections: none
- Follow-up TODOs: none
-->

# PocketMic Constitution

## Core Principles

### I. Real-Time Determinism & Lock-Free Audio Thread (NON-NEGOTIABLE)
El hilo de procesamiento de audio en tiempo real (*Audio Callback Thread*) debe ser estrictamente determinista y libre de bloqueos.
- Queda terminantemente PROHIBIDO realizar llamadas bloqueantes, asignaciones dinámicas de memoria en el heap (`malloc`, `free`, `new`), operaciones de E/S de disco/red, bloqueos por mutex o llamadas a APIs del sistema operativo no seguras para tiempo real dentro del callback de audio.
- Toda la comunicación y transferencia de datos entre el hilo de red/UI y el hilo de audio DSP DEBE realizarse mediante colas circulares lock-free (*Single-Producer Single-Consumer RingBuffers*).
- Violaciones de esta regla causan caídas de buffers (*underruns*) y clicks audibles inaceptables.

### II. Latency Budget & Zero Bloat (NON-NEGOTIABLE)
PocketMic debe mantenerse como una herramienta de ultra-bajo consumo de recursos y latencia imperceptible:
- **Presupuesto de Latencia:** La latencia total end-to-end DEBE ser $\le 12$ ms en Wi-Fi (5 GHz) y $\le 5$ ms en conexión por cable USB.
- **Presupuesto de CPU:** El cliente de escritorio DEBE consumir $\le 2.0\%$ de CPU en equipos modernos de 4 núcleos. La inferencia del modelo neuronal (DeepFilterNet 3) DEBE suspenderse automáticamente a 0% de CPU mediante un Voice Activity Detector (VAD) cuando no se detecte actividad vocal durante más de 1.5 segundos.
- **Huella de Memoria:** El daemon de audio no excederá los 45 MB de memoria RAM en reposo. Cero dependencias de navegadores embebidos pesados o frameworks hinchados en el daemon de sonido.

### III. Spec-Driven & Test-First (TDD / SDD)
Todo componente del sistema debe especificarse antes de ser programado y verificarse mediante pruebas automatizadas:
- Se prohíbe el desarrollo no guiado ("vibe coding") en componentes del núcleo (protocolo de transporte UDP, decodificador Opus, filtros DSP, cuantizador de autotune y formateador de paquetes).
- Las especificaciones en Spec Kit (`/speckit-specify` y `/speckit-plan`) son el contrato fuente de verdad.
- Cada módulo DSP o de red DEBE contar con pruebas unitarias que validen casos de borde (pérdida de paquetes, jitter de red, señales de saturación/clipping y conmutación de presets) antes de su integración.

### IV. High Acoustic Fidelity (Float32 @ 48 kHz)
La calidad acústica nunca debe sacrificarse en aras de la simplificación:
- El pipeline interno de procesamiento DSP DEBE operar con resolución de coma flotante de 32 bits (`f32`) a 48 kHz, garantizando un margen dinámico infinito sin distorsión por cuantización ni truncamiento de armónicos.
- La compresión de transporte DEBE utilizar el códec Opus optimizado para voz de baja latencia (frames de 5 a 10 ms), con soporte de FEC (*Forward Error Correction*) y mitigación de pérdidas por PLC (*Packet Loss Concealment*).

### V. Resilient Connectivity & Zero Friction
La conexión entre el teléfono y la computadora debe ser robusta frente a condiciones adversas de red y hardware:
- El sistema DEBE implementar auto-descubrimiento en red local mediante UDP Broadcast, acompañado OBLIGATORIAMENTE de un mecanismo de conexión directa manual por IP o PIN de 4 dígitos para entornos con aislamiento de clientes (*AP Isolation* en hoteles, universidades o redes empresariales).
- La conexión por cable USB en Android DEBE operar sobre el protocolo estándar **AOA** (Android Open Accessory) y en iOS sobre el túnel **usbmuxd**, garantizando una experiencia Plug & Play sin requerir habilitar opciones de desarrollador ni comandos de consola en el dispositivo del usuario.

## Technology Stack & Hardware Boundary Constraints

1. **Host de Escritorio (Desktop Engine):**
   - Lenguaje principal: **Rust** para el daemon de audio en tiempo real, sockets de red y runtime de inferencia ONNX.
   - Plataformas objetivo de primer nivel: Windows 10/11 (WASAPI / driver virtual de audio) y Linux (nodo nativo en PipeWire `Audio/Source`).
2. **Cliente Móvil (Android / iOS):**
   - Motor de captura acústica: **C++ nativo** utilizando **Google Oboe** en Android (prioridad AAudio con fallback a OpenSL ES) y **CoreAudio RemoteIO** en iOS.
   - Interfaz de usuario: Nativa en Kotlin / Swift para un rendimiento táctil óptimo en la cuadrícula de efectos (*Stream Deck*).
3. **Protocolo de Red:**
   - Datagramas UDP directos con cabecera binaria propietaria de 16 bytes (`Magic: 0x504D`).

## Quality Gates & Performance Benchmarking

- **Audio Integrity Gate:** Ningún cambio de código puede integrarse si introduce bloqueos de sincronización o demoras mayores a 10 ms en el procesamiento de frames de audio.
- **DSP Verification Gate:** Toda función matemática de audio (filtro High-Pass, ecualizador de 3 bandas, compresor dinámico, pitch tracker YIN y formantes) debe ser verificada frente a señales sintéticas de prueba (barridos senoidales 20 Hz–20 kHz, pulsos Dirac y ruido rosa) comprobando estabilidad numérica y respuesta de fase.
- **Network Degradation Gate:** El subsistema de transporte debe ser sometido a pruebas de estrés simulando hasta un 5% de pérdida de paquetes y 30 ms de jitter sin provocar crashes ni desbordamientos de buffer.

## Governance

- Esta Constitución es la ley técnica suprema de PocketMic y prevalece sobre cualquier decisión informal de implementación.
- Cualquier modificación a los principios rectores requiere:
  1. Justificación técnica documentada en un Registro de Decisión de Arquitectura (ADR).
  2. Aprobación y actualización formal mediante `/speckit-constitution`.
  3. Plan de migración o ajuste de los presupuestos de rendimiento afectados.
- Toda revisión de pull request y tarea ejecutada mediante `/speckit-implement` DEBE validar explícitamente el cumplimiento de estos principios.

**Version**: 1.0.0 | **Ratified**: 2026-11-09 | **Last Amended**: 2026-11-09
