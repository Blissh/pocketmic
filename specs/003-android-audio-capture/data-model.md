# Data Model: Android Audio Capture & Mobile Client

**Feature**: `003-android-audio-capture`  
**Date**: 2026-11-09  
**Status**: Completed  

---

## 1. Entidades del Cliente Móvil

### 1.1. `MicCapsule` (Cápsulas Físicas Disponibles)
```kotlin
enum class MicCapsule {
    BOTTOM_DEFAULT,  // Modo mano alzada / conferencia horizontal
    FRONT_TOP,       // Modo atril / soporte de escritorio apuntando a la boca
    AUTO_SELECT      // Orientación adaptativa por acelerómetro
}
```

---

### 1.2. `MobileCaptureConfig` (Configuración de Captura Nativa)
```kotlin
data class MobileCaptureConfig(
    val sampleRate: Int = 48000,
    val channelCount: Int = 1,
    val frameDurationMs: Int = 10,
    val capsule: MicCapsule = MicCapsule.BOTTOM_DEFAULT,
    val sidetoneEnabled: Boolean = false,
    val sidetoneVolume: Float = 0.8f, // 0.0 a 1.0
    val targetHostIp: String? = null,
    val targetPort: Int = 37719
)
```

---

### 1.3. `MobileSessionState` (Estado de la App Móvil)
```kotlin
sealed class MobileSessionState {
    object Idle : MobileSessionState()
    object Discovering : MobileSessionState()
    data class Connected(
        val hostName: String,
        val hostIp: String,
        val audioPort: Int,
        val isMuted: Boolean,
        val activeEffect: Int
    ) : MobileSessionState()
    data class Reconnecting(val attempts: Int) : MobileSessionState()
    data class Error(val message: String) : MobileSessionState()
}
```

---

### 1.4. `StreamDeckAction` (Acciones Rápidas Táctiles en Pantalla)
```kotlin
data class StreamDeckPad(
    val id: String,
    val label: String,
    val iconRes: String,
    val effectId: Int,
    val isToggle: Boolean,
    var isActive: Boolean = false
)
```
