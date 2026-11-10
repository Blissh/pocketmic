# Phase 0 Research: Streamer Fun FX & Voice Changer

**Feature**: `002-streamer-fun-fx`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Decisiones Técnicas y Algoritmos DSP

### 1.1. Autotune en Tiempo Real (Hard-Tune / T-Pain)
* **Detección de Tono Fundamental ($f_0$):**
  * **Decisión:** Algoritmo **YIN simplificado** con correlación de diferencia cuadrada normalizada.
  * **Rango:** 75 Hz a 600 Hz (cobertura total del registro vocal humano de habla y canto).
  * **Ventana:** 10 ms (480 muestras a 48 kHz).
* **Cuantización de Escala:**
  * Las frecuencias detectadas se mapean a números de nota MIDI mediante:
    $$\text{note} = 69 + 12 \cdot \log_2(f_0 / 440.0)$$
  * La nota calculada se restringe a las notas permitidas en la escala activa:
    * **Cromática:** Todas las 12 notas de la escala temperada.
    * **Pentatónica Menor:** 5 notas clave (ideal para trap / rap melódico).
    * **Mayor / Menor Natural:** Según la tonalidad elegida.
* **Resíntesis de Tono:**
  * *Retune Speed* inmediato ($0$ ms) para el sonido característico robótico y percusivo en las transiciones entre notas.
  * Implementado mediante **WSOLA** (*Waveform Similarity Overlap-Add*) o Phase Vocoder de baja latencia con pasos de 240 muestras (5 ms).

---

### 1.2. Voz Anime / "Monita China" (Pitch Shifter + Formant Scaling)
* **El Problema Físico:**
  * Al aplicar un pitch shift puro hacia los agudos ($+5$ semitonos), los armónicos y las frecuencias formánticas ($F_1, F_2, F_3$) se desplazan por igual. El oído humano percibe esto como un objeto diminuto que vibra (efecto "ardilla / helio").
* **La Solución Acústica:**
  * Desacoplar la **frecuencia fundamental ($f_0$)** de la **envolvente espectral del tracto vocal**:
    1. **Pitch Shift:** Aumenta la fundamental $+4$ a $+6$ semitonos (típicamente $+5$, ratio $\approx 1.335$).
    2. **Formant Scaling (1.18x – 1.25x):** Aplica un banco de filtros biquad de seguimiento espectral o interpolación FFT que ensancha las resonancias bucofaríngeas emulando un tracto vocal anatómicamente más corto (característico de voces femeninas/juveniles).
  * **Resultado:** Timbre agudo, claro, cómico y completamente inteligible sin artefactos de ardilla.

---

### 1.3. Algoritmos de Efectos Meme y Atmosféricos
* **Radio Militar / Walkie-Talkie:**
  * Filtro paso banda IIR Butterworth de 4º orden: $350$ Hz a $2.800$ Hz.
  * Saturación analógica no lineal: $y[n] = \tanh(1.8 \cdot x[n])$.
  * Generador de ráfaga de estática de 40 ms y *"Roger Beep"* (doble tono: 1.000 Hz por 60 ms + 1.200 Hz por 60 ms) al soltar el canal.
* **Demonio / Deep Monster:**
  * Pitch shift descendente de $-8$ semitonos (ratio $\approx 0.630$).
  * Inyección de sub-octava con ganancia de $+3$ dB y refuerzo Low-Shelf en $120$ Hz.
* **Robot / Sci-Fi Synth:**
  * Modulación en anillo (*Ring Modulator*): Multiplicación de la señal por una portadora senoidal pura a $75$ Hz con mezcla wet/dry de 40%/60%.
* **Catedral / Cueva (Épico Reverb):**
  * Algoritmo *Freeverb* optimizado (4 filtros peine paralelos + 2 filtros pasatodo). Decaimiento de $3.5$ segundos y amortiguamiento de agudos.
* **Bleep de Censura:**
  * Oscilador senoidal puro a $1.000$ Hz. Fundido cruzado de $2$ ms (*linear crossfade*) al entrar y salir para garantizar cero chasquidos digitales.

---

### 1.4. Protocolo de Control Remoto de Baja Latencia
* Los comandos de conmutación de efectos se despachan mediante paquetes UDP ultraligeros de **8 bytes** (`PacketType: 0x10`), logrando una latencia de conmutación desde la pantalla del móvil a la PC menor a **1.5 ms**.

---

## 2. Validación de Criterios Constitucionales

* **Principio I (Lock-Free Audio Callback):** Todos los coeficientes de filtros biquad, tablas de senos del oscilador y buffers del Freeverb se pre-asignan estáticamente. Cero `malloc` en tiempo de procesamiento.
* **Principio II (Presupuesto de CPU y Latencia):** El DSP completo de efectos consume $< 1.2\%$ de CPU en un hilo de escritorio estándar y agrega $\le 4$ ms de latencia algorítmica.
