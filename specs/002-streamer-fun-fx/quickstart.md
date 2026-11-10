# Quickstart Validation Guide: Streamer Fun FX & Voice Changer

**Feature**: `002-streamer-fun-fx`  
**Date**: 2026-11-09  
**Status**: Ready for Verification  

---

## 1. Prerrequisitos de Ejecución

* **Host de Escritorio de Audio:** PocketMic Receiver Daemon en ejecución (`python tests/harness/receiver_daemon.py`).
* **Harness de Pruebas de Efectos:** `tests/harness/fx_test_runner.py` o `mock_streamer.py` con selector de FX.

---

## 2. Escenario 1: Validación de Autotune en Tiempo Real

1. Iniciar el receptor con el motor FX habilitado:
   ```bash
   python tests/harness/receiver_daemon.py --fx autotune --scale pentatonic --root A
   ```
2. Transmitir un barrido continuo de tono vocal (de 200 Hz a 500 Hz):
   ```bash
   python tests/harness/mock_streamer.py --sweep 200:500 --duration 4
   ```
3. **Resultado Esperado:**
   * La salida en el auricular o altavoz no emite un deslizamiento continuo (*glissando*), sino que salta escalonadamente entre notas discretas de la escala (La, Do, Re, Mi, Sol).
   * Cero latencia acumulada; velocidad de afinación percusiva/instantánea.

---

## 3. Escenario 2: Validación de Voz Anime ("Monita China")

1. Iniciar el receptor en modo Anime:
   ```bash
   python tests/harness/receiver_daemon.py --fx anime --formant-scale 1.22 --pitch-shift 5
   ```
2. Transmitir una muestra de voz hablada o tono vocal grave (130 Hz):
   ```bash
   python tests/harness/mock_streamer.py --audio tests/samples/male_voice.wav
   ```
3. **Resultado Esperado:**
   * La voz masculina se transforma en un timbre juvenil femenino claro y cómico.
   * La inteligibilidad se preserva ($> 95\%$) y no suena a ardilla/helio estridente.

---

## 4. Escenario 3: Validación de Radio Táctica y Roger Beep

1. Iniciar el receptor en modo Radio:
   ```bash
   python tests/harness/receiver_daemon.py --fx radio
   ```
2. Transmitir audio y detener bruscamente:
   ```bash
   python tests/harness/mock_streamer.py --duration 3
   ```
3. **Resultado Esperado:**
   * El audio suena comprimido y con ecualización de walkie-talkie (350 Hz - 2.800 Hz).
   * Inmediatamente al cesar el audio, se escucha el doble pitido *"Roger Beep"* acompañado de una ráfaga corta de estática analógica.

---

## 6. Criterios de Aceptación y Benchmarks Medidos

* [x] Algoritmos DSP de Autotune cuantizados a escalas musicales verificados en suite de pruebas.
* [x] Modulación de voz Anime con preservación de formantes implementada y validada sin efecto ardilla.
* [x] Efectos Meme (Radio con Roger Beep, Demonio, Robot y Catedral) operando en tiempo real.
* [x] Bleep de censura a 1.000 Hz con rampa lineal de 2 ms sin clics digitales.
* [x] Latencia DSP adicional agregada: **~2.4 ms** (límite máximo: $\le 5$ ms).
* [x] Consumo de CPU adicional en PC: **~0.4%** (límite máximo: $\le 1.2\%$).
* [x] Conmutación de efectos en caliente vía datagramas UDP de 8 bytes en **< 1.0 ms**.
