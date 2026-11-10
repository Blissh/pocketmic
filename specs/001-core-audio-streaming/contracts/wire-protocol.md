# Wire Protocol Specification: PocketMic Core (v1)

**Protocolo:** UDP / Binary  
**Puertos por defecto:**
* Puerto de Descubrimiento (Broadcast): `37718`
* Puerto de Datos y Control de Audio: `37719`

---

## 1. Datagrama de Audio (`Packet Type = 0x01`)

Estructura fija de 16 bytes de encabezado + N bytes de payload Opus:

| Offset (Bytes) | Campo | Tipo | Descripción |
| :---: | :--- | :---: | :--- |
| `0..2` | `magic` | `u16` (Big-Endian) | `0x504D` (ASCII `'P' 'M'`) |
| `2..3` | `version` | `u8` | Versión del protocolo (`0x01`) |
| `3..4` | `packet_type` | `u8` | `0x01` = Audio Data |
| `4..8` | `sequence_number`| `u32` (Big-Endian) | Contador monótono incremental (`0, 1, 2, ...`) |
| `8..12` | `timestamp_us` | `u32` (Big-Endian) | Marca temporal de captura en microsegundos |
| `12..13` | `flags` | `u8` | Bitfield: `0x01`: Mute, `0x02`: FEC, `0x04`: Clipping |
| `13..14` | `channels` | `u8` | `0x01` = Mono, `0x02` = Estéreo |
| `14..16` | `payload_len` | `u16` (Big-Endian) | Tamaño exacto en bytes del bloque Opus |
| `16..16+N`| `payload` | `[u8]` | Datos de audio comprimidos en códec Opus |

---

## 2. Mensajes de Descubrimiento y Handshake

### 2.1. `DISCOVER_REQ` (`Packet Type = 0x04`)
Enviado por el móvil por broadcast UDP (`255.255.255.255:37718`):

```json
{
  "cmd": "DISCOVER_REQ",
  "client_id": "phone-uuid-v4",
  "device_name": "Pixel 9 Pro",
  "protocol_version": 1
}
```

### 2.2. `DISCOVER_ACK` (`Packet Type = 0x05`)
Respuesta del servidor PC en unicast directo a la IP del móvil:

```json
{
  "cmd": "DISCOVER_ACK",
  "server_name": "Gaming-PC-Windows",
  "audio_port": 37719,
  "protocol_version": 1,
  "supported_codecs": ["opus"],
  "sample_rate": 48000
}
```

### 2.3. `HANDSHAKE_INIT` (`Packet Type = 0x06`)
Enviado por el móvil a la PC (`IP:37719`):

```json
{
  "cmd": "HANDSHAKE_INIT",
  "client_id": "phone-uuid-v4",
  "channels": 1,
  "frame_duration_ms": 10,
  "initial_bitrate_kbps": 64
}
```

### 2.4. `HANDSHAKE_OK` (`Packet Type = 0x07`)
Confirmación de la PC lista para recibir el stream de voz:

```json
{
  "cmd": "HANDSHAKE_OK",
  "session_token": "token-hex-64"
}
```

---

## 3. Mensajes de Control en Tiempo Real (`Packet Type = 0x10`)

Estructura binaria ultra-compacta de 8 bytes para latencia $< 1$ ms:

```
[Magic: 0x504D] [Ver: 0x01] [Type: 0x10] [CMD_ID: u8] [Param1: u16]
```

* `CMD_ID`:
  * `0x01`: Set Mute (`Param1 = 0` Unmute, `Param1 = 1` Mute).
  * `0x02`: Set Volume Pre-Gain (`Param1 = 0..1000` milésimas de ganancia).
  * `0x03`: Set Bitrate Request (`Param1 = kbps`, ej. 32, 64, 96).
  * `0x04`: Ping Telemetría (`Param1 = SeqNum`).
