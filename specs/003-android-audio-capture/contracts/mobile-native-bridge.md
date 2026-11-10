# Interface Contract: Android Mobile Native Bridge (JNI)

**Mecanismo:** Java Native Interface (JNI) C++ bindings  
**Namespace C++:** `pocketmic::audio`  
**Clase Kotlin:** `io.pocketmic.core.AudioEngineBridge`  

---

## 1. Métodos Nativos JNI

```kotlin
package io.pocketmic.core

object AudioEngineBridge {
    init {
        System.loadLibrary("pocketmic_engine")
    }

    /**
     * Inicializa el motor Oboe y el encoder Opus.
     * @return 0 en éxito, código de error negativo en fallo.
     */
    external fun nativeInit(sampleRate: Int, channels: Int): Int

    /**
     * Comienza la captura en tiempo real y el streaming UDP hacia el host de PC.
     */
    external fun nativeStart(hostIp: String, port: Int): Int

    /**
     * Detiene la captura y libera el socket.
     */
    external fun nativeStop(): Int

    /**
     * Conmuta la cápsula física del micrófono (0 = Inferior, 1 = Frontal/Superior).
     */
    external fun nativeSetMicrophoneDirection(direction: Int): Int

    /**
     * Activa o desactiva el sidetone local en auriculares.
     */
    external fun nativeSetSidetone(enabled: Boolean, volume: Float): Int

    /**
     * Silencia o activa el flujo de audio (Mute).
     */
    external fun nativeSetMute(muted: Boolean): Int

    /**
     * Despacha un comando binario de cambio de efecto (FX) a la PC.
     */
    external fun nativeSendFxCommand(effectId: Int, mode: Int, param1: Int): Int

    /**
     * Consulta el nivel pico actual (0.0 a 1.0) para el VU Meter.
     */
    external fun nativeGetPeakLevel(): Float
}
```

---

## 2. Eventos Callback desde C++ hacia Kotlin

```kotlin
interface AudioEngineCallback {
    fun onConnectionLost()
    fun onClippingAlert(peakDbfs: Float)
    fun onNetworkStatsUpdated(packetsSent: Long, lossRate: Float)
}
```
