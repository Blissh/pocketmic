# Phase 0 Research: Core Audio Streaming

**Feature**: `001-core-audio-streaming`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Decisiones Tecnológicas y Mitigación de Riesgos

### 1.1. Backend de Salida de Audio en Escritorio (Windows & Linux)
* **Decisión:** Utilizar el crate de Rust **`cpal`** (Cross-Platform Audio Library) configurado con el host nativo **WASAPI** en Windows y **PipeWire / ALSA** en Linux.
* **Justificación:**
  * Proporciona acceso directo al callback de audio del sistema operativo sin introducir capas de emulación lentas.
  * Soporta nativamente streams en formato `f32` a 48 kHz.
  * Permite seleccionar el dispositivo de salida específico (ej. un cable virtual tipo VB-Audio Cable o la salida de auriculares directa para pruebas de monitoreo).
* **Alternativas evaluadas:**
  * *PortAudio C bindings:* Descartado por requerir binarios externos (`portaudio.dll`) difíciles de empaquetar de forma autocontenida en Windows.
  * *IAudioClient COM APIs directo en Rust:* Descartado para el MVP debido a complejidad excesiva de boilerplate COM; `cpal` ya implementa WASAPI internamente con optimización para baja latencia.

---

### 1.2. Decodificación Opus y Manejo de Paquetes
* **Decisión:** Utilizar **`opus`** / **`audiopus`** con perfil `OPUS_APPLICATION_RESTRICTED_LOWDELAY` o `VOIP`, con frames de **10 ms** (480 muestras a 48 kHz).
* **Justificación:**
  * La latencia algorítmica de Opus a frames de 10 ms es de apenas 5 ms.
  * Soporta PLC (*Packet Loss Concealment*): si un datagrama UDP se pierde por interferencia de red, el decodificador genera un frame sintético suavizado, evitando clicks y chasquidos metálicos.
* **Alternativas evaluadas:**
  * *PCM Float32 sin comprimir:* Descartado porque requiere ~1.536 kbps de ancho de banda constante; en redes Wi-Fi con saturación provoca ráfagas de pérdida de paquetes inaceptables.
  * *AAC-ELD:* Descartado por licenciamiento restrictivo y mayor sobrecarga de codificación.

---

### 1.3. Jitter Buffer Lock-Free (Audio Thread Safety)
* **Decisión:** Utilizar una cola circular **Single-Producer Single-Consumer (SPSC)** basada en la librería **`ringbuf`**.
* **Justificación:**
  * **Cumplimiento estricto con la Constitución (Principio I):** El hilo de audio de CPAL solo lee de la cola sin adquirir ningún mutex ni realizar `malloc`/`free`.
  * El hilo de red UDP actúa como único productor escribiendo los frames decodificados.
  * Capacidad fija pre-asignada (ej. 4 buffers de 10 ms = 40 ms de colchón máximo ajustable dinámicamente) para absorber la fluctuación de la Wi-Fi sin retrasar la voz.
* **Alternativas evaluadas:**
  * `Arc<Mutex<VecDeque<f32>>>`: Terminantemente descartado por violar la Constitución del proyecto (riesgo de inversión de prioridad y caídas de buffer).

---

### 1.4. Wire Protocol & Socket UDP Directo
* **Decisión:** Datagramas UDP directos con cabecera de 16 bytes (`Magic 0x504D`). Sockets en modo non-blocking o en hilo dedicado con `SO_RCVBUF` ampliado a 64 KB.
* **Justificación:**
  * Cero sobrecarga de protocolos complejos (evita el handshake de 4 vías de WebRTC o el head-of-line blocking de TCP).
  * Cabecera suficiente para detectar pérdida con `sequence_number` monótono y calcular jitter con `timestamp_us`.
* **Alternativas evaluadas:**
  * *WebRTC DataChannels / MediaStream:* Descartado para el MVP por sobrecarga masiva de señalización (SDP, STUN/TURN, DTLS).

---

## 2. Conclusiones y Validación de Gates

Todas las incógnitas técnicas han sido resueltas con opciones tecnológicas concretas y compatibles con los presupuestos de latencia ($\le 12$ ms) y los mandatos de tiempo real de la Constitución de PocketMic.
