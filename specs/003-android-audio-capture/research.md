# Phase 0 Research: Android Audio Capture & Mobile Client

**Feature**: `003-android-audio-capture`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Decisiones Tecnológicas y Mitigación de Riesgos

### 1.1. Motor de Captura Acústica de Baja Latencia: Google Oboe (C++)
* **Decisión:** Implementar el motor de captura en **C++ nativo utilizando Google Oboe** mediante el Android NDK.
* **Justificación:**
  * La API `AudioRecord` estándar de Java/Kotlin introduce una sobrecarga del recolector de basura (GC) y un puente JNI por cada buffer que añade de 15 a 30 ms de latencia variable.
  * Oboe selecciona automáticamente la mejor API nativa del dispositivo: **AAudio** (en Android 8.0+) para latencias de hardware puras de 3 a 5 ms, con fallback transparente a **OpenSL ES** en dispositivos antiguos.
  * El hilo de captura nativo opera con prioridad de tiempo real `THREAD_PRIORITY_URGENT_AUDIO`.

---

### 1.2. Selección de Cápsula de Micrófono Física
* **Decisión:** Utilizar `MicrophoneInfo` y la API `setPreferredMicrophoneDirection` / `setPreferredDevice` (Android 9.0+ / API 28+).
* **Mapeo:**
  * **Modo Mano Alzada:** `MIC_DIRECTION_UNSPECIFIED` o cápsula inferior (`TYPE_BUILTIN_MIC`).
  * **Modo Atril / Soporte:** `MIC_DIRECTION_TOWARDS_USER` solicitando la cápsula frontal/superior.
* **Fallback:** Si el hardware no expone múltiples micrófonos lógicos, el motor continúa con la cápsula principal sin generar errores.

---

### 1.3. Sidetone Local con Latencia Cero en Audífonos
* **Decisión:** Dentro del callback de captura de Oboe (`onAudioReady`), si hay auriculares detectados por `AudioManager` (`TYPE_WIRED_HEADPHONES`, `TYPE_USB_HEADSET` o Bluetooth), copiar directamente una réplica del buffer adquirido hacia un flujo de salida `OboeAudioStream` de salida rápida.
* **Latencia:** Inmediata ($< 2$ ms en auriculares por cable), evitando la sensación de oído tapado sin enviar nada por la red.
* **Seguridad:** Silenciado automático inmediato si los auriculares se desconectan para evitar retroalimentación acústica (*feedback/acople*) por los altavoces del teléfono.

---

### 1.4. Persistencia en Segundo Plano (Foreground Service)
* **Decisión:** Declarar e iniciar un `ForegroundService` con `android:foregroundServiceType="microphone"` en Android 10+ (API 29+).
* **Justificación:**
  * Los sistemas de ahorro de batería agresivos de Android (Doze Mode, MIUI/HyperOS, Samsung OneUI) matan cualquier proceso en segundo plano que grabe audio si no cuenta con notificación permanente y tipo de servicio declarado.

---

### 1.5. Codificación Opus Nativa en el Móvil
* **Decisión:** Compilar `libopus` estáticamente en el NDK de Android.
* **Parámetros:** 48 kHz, 1 canal (mono), frame de 10 ms (480 muestras), aplicación `OPUS_APPLICATION_VOIP` o `RESTRICTED_LOWDELAY`.
* **Rendimiento:** Cada frame de 10 ms se codifica en menos de 0.25 ms en procesadores ARM64 modernos, manteniendo el consumo de CPU por debajo del 3%.

---

## 2. Validación de Puertas Constitucionales

* **Principio I (Lock-Free Audio Thread):** El callback de Oboe solo copia muestras a la cola circular y al encoder Opus; cero asignaciones dinámicas en Java o C++ en caliente.
* **Principio II (Presupuesto de Rendimiento):** Latencia de captura en móvil $< 6$ ms; consumo de batería $< 5\%$ por hora.
