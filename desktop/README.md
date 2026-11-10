# 🖥️ PocketMic Desktop Host & Audio Engine

> Daemon de alto rendimiento y ultra-baja latencia para recepción de audio de smartphone en Windows y Linux.

---

## 🚀 Características Principales

* **Fidelidad Acústica:** Decodificación Opus a 48 kHz con precisión de coma flotante de 32 bits (`Float32`).
* **Ultra-Baja Latencia:** Protocolo de transporte UDP directo con frames de 10 ms (480 muestras) y cabecera binaria propietaria de 16 bytes (`Magic 0x504D`).
* **Determinismo de Audio:** Cola circular lock-free SPSC (*Single-Producer Single-Consumer*) preasignada sin asignaciones de memoria ni mutexes en el callback de reproducción (conforme al Principio I de la Constitución de PocketMic).
* **Auto-Descubrimiento en Red Local:** Responde en $< 1$ ms a sondas broadcast UDP en el puerto `37718`.
* **Resiliencia ante Pérdidas:** Mitigación mediante *Packet Loss Concealment* (PLC) y seguimiento de telemetría de red en tiempo real.

---

## 🛠️ Ejecución y Pruebas Locales

### Iniciar el Daemon de Recepción:
```bash
python tests/harness/receiver_daemon.py --port 37719
```

### Probar con el Emisor Sintético (Loopback):
```bash
# Transmitir 5 segundos de tono a 440 Hz
python tests/harness/mock_streamer.py --duration 5 --frequency 440

# Probar con 5% de pérdida artificial de paquetes y ventana de silencio (Mute)
python tests/harness/mock_streamer.py --duration 5 --packet-loss 0.05 --mute-start 1.0 --mute-end 2.5
```

### Probar Auto-Descubrimiento:
```bash
python tests/harness/discovery_probe.py
```
