# Interface Contract: Streamer FX Control Protocol (v1)

**Protocolo:** UDP / Binary  
**Puerto:** `37719` (Mismo socket que el canal de datos de audio)  
**Tamaño del Datagrama:** Exactamente 8 bytes  

---

## 1. Estructura Binaria de Comandos de Efectos (`Type = 0x10`)

| Offset (Bytes) | Campo | Tipo | Valor / Descripción |
| :---: | :--- | :---: | :--- |
| `0..2` | `magic` | `u16` (Big-Endian) | `0x504D` (ASCII `'P' 'M'`) |
| `2..3` | `version` | `u8` | `0x01` |
| `3..4` | `packet_type` | `u8` | `0x10` (FX_CMD) |
| `4..5` | `effect_id` | `u8` | ID del efecto objetivo (ver tabla abajo) |
| `5..6` | `mode` | `u8` | `0x00`: Apagado, `0x01`: Encendido (Toggle), `0x02`: Push-to-Effect |
| `6..8` | `param1` | `u16` (Big-Endian) | Parámetro numérico específico del efecto |

---

## 2. Mapa de Efectos y Parámetros

| `effect_id` | Nombre del Efecto | Significado de `param1` | Valores Típicos |
| :---: | :--- | :--- | :--- |
| `0x00` | **Bypass (Voz Normal)** | Ninguno (0) | `0` |
| `0x01` | **Autotune** | Byte alto: Escala (`0` Cromática, `1` Mayor, `2` Menor, `3` Pentatónica)<br>Byte bajo: Nota Raíz (`0`=Do, `2`=Re, `9`=La...) | `0x0309` (Pentatónica de La) |
| `0x02` | **Anime / "Monita China"** | Formant scale $\times 100$ (ej. $120$ para $1.20\times$) | `120` |
| `0x03` | **Ardilla / Chipmunk** | Pitch shift semitonos $\times 10$ (ej. $+80$ para $+8.0$ st) | `80` |
| `0x04` | **Demonio / Deep Monster**| Pitch shift semitonos $\times 10$ negativo (ej. $-80$) | `0xFFB0` (complemento a 2) |
| `0x05` | **Radio Militar Táctica** | Roger Beep: `1` habilitado, `0` deshabilitado | `1` |
| `0x06` | **Robot Sci-Fi** | Frecuencia portadora en Hz | `75` |
| `0x07` | **Megáfono** | Drive de saturación | `2` |
| `0x08` | **Catedral Reverb** | Tamaño de sala / Decaimiento ($0..100$) | `85` |
| `0x09` | **Bleep de Censura** | Estado: `1` = Pitido activo, `0` = Inactivo | `1` |

---

## 3. Comportamiento en Tiempo Real

* **Fundido Cruzado (Crossfade):** Al recibir cualquier `FX_CMD`, el motor de audio aplica un fundido cruzado lineal de 96 muestras (2 ms a 48 kHz) entre el efecto saliente y el entrante.
* **Prioridad Absoluta de Censura:** El comando `Bleep` (`0x09`) anula temporalmente cualquier otro efecto activo mientras se mantenga en estado `1`. Al soltarse (`0`), se restaura automáticamente el efecto previo.
