# Feature Specification: Streamer Fun FX & Voice Changer

**Feature Branch**: `002-streamer-fun-fx`

**Created**: 2026-11-09

**Status**: Draft

**Input**: User description: "Streamer Fun FX: Modulación de voz en tiempo real para streamers (Autotune, Anime/VTuber con escalado de formantes, Radio militar, Robot, Demonio, Catedral y Bleep de censura)"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Autotune melódico y robótico en tiempo real (Priority: P1)

Como streamer o jugador en Discord/Twitch,  
quiero que mi voz se corrija automáticamente a notas musicales afinadas al hablar o cantar,  
para poder hacer bromas musicales, cantar estilo trap/T-Pain o dar un tono cómico a mis transmisiones sin retardo perceptible.

**Why this priority**: Es uno de los efectos más populares y solicitados por creadores de contenido para momentos cómicos y clips virales.

**Independent Test**: Hablar o cantar en el micrófono con el efecto activado; la voz se debe escuchar afinada instantáneamente a la nota de la escala musical elegida (ej. Pentatónica o Cromática) con el clásico timbre metálico de autotune.

**Acceptance Scenarios**:

1. **Given** que el usuario activa el modo Autotune, **When** canta o habla variando su tono, **Then** la salida de audio salta de forma instantánea a la nota musical más cercana de la escala sin desafinaciones intermedias.
2. **Given** que el usuario cambia la escala musical, **When** continúa hablando, **Then** la corrección de tono se adapta de inmediato a las nuevas notas permitidas.

---

### User Story 2 - Modulación de voz Anime / "Monita China" con timbre natural (Priority: P1)

Como creador de contenido, VTuber o usuario de juegos en línea,  
quiero transformar mi voz para que suene como un personaje femenino o juvenil de anime sin el desagradable efecto "ardilla/helio",  
para entretener a mi audiencia con un personaje cómico e inteligible.

**Why this priority**: Los moduladores de voz tradicionales solo suben el tono (*pitch*), haciendo que el usuario suene como si hubiera inhalado helio. La modulación de anime con preservación/escalado de formantes es un diferenciador clave que no existe en soluciones ligeras gratuitas.

**Independent Test**: Hablar con voz grave o media con el preset Anime activado; la voz debe transformarse en un tono agudo y juvenil femenino, siendo 100% inteligible y libre de distorsiones de ardilla.

**Acceptance Scenarios**:

1. **Given** que el preset Anime está activo, **When** un usuario masculino con voz grave habla al micrófono, **Then** la voz resultante suena femenina, juvenil y clara, manteniendo la articulación exacta de las palabras.
2. **Given** que el usuario habla a diferentes volúmenes, **When** susurra o habla fuerte, **Then** la transformación se mantiene estable sin generar cortes ni artefactos estridentes.

---

### User Story 3 - Efectos Meme y Atmosféricos de Transmisión (Priority: P2)

Como jugador de títulos tácticos o streamer de rol,  
quiero conmutar rápidamente entre efectos temáticos (Radio Militar con "Roger Beep", Demonio grave, Robot sci-fi y Catedral con eco épico),  
para ambientar mis partidas y reaccionar de forma cómica ante situaciones del juego.

**Why this priority**: Aporta variedad y dinamismo al catálogo de efectos sin sobrecargar la interfaz.

**Independent Test**: Conmutar entre los distintos presets mientras se habla, verificando que la radio suene comprimida con estática y sonido de confirmación al soltar, el demonio con cuerpo subgrave, y la catedral con reverberación espaciosa.

**Acceptance Scenarios**:

1. **Given** que el preset Radio Táctica está activo, **When** el usuario termina de hablar y suelta la transmisión, **Then** se reproduce automáticamente una ráfaga corta de estática y un pitido ("Roger Beep").
2. **Given** que el usuario conmuta entre dos efectos distintos mientras habla, **When** se produce el cambio, **Then** la transición es suave sin chasquidos ni caídas de volumen abruptas.

---

### User Story 4 - Botón de censura rápida (*Push-to-Bleep*) (Priority: P3)

Como streamer en vivo sujeto a reglas de plataformas (Twitch, YouTube, Kick),  
quiero mantener presionado un botón en pantalla que tape inmediatamente mi voz con el clásico pitido de censura de televisión,  
para censurar palabras inapropiadas o momentos de frustración de manera graciosa y segura.

**Why this priority**: Es una herramienta clásica de comedia para streamers y añade valor directo en situaciones de riesgo de moderación de contenido.

**Independent Test**: Mantener presionado el botón Bleep en pantalla; la voz debe quedar 100% sustituida por un tono senoidal a 1.000 Hz hasta que se suelte el botón.

**Acceptance Scenarios**:

1. **Given** que el usuario está transmitiendo en vivo, **When** mantiene presionado el botón "BLEEP", **Then** el audio del micrófono se silencia por completo y es sustituido por un pitido de 1.000 Hz.
2. **Given** que el usuario suelta el botón "BLEEP", **When** retoma la palabra, **Then** la voz vuelve a transmitirse de forma instantánea sin demora.

---

### Edge Cases

- **Conmutación rápida entre efectos:** Si el usuario pulsa múltiples efectos en menos de 100 ms, el sistema debe encolar o aplicar el último estado con un fundido cruzado (*crossfade*) de 2 ms para prevenir clicks digitales.
- **Rango vocal extremo:** Si el usuario grita o emite sonidos inarmónicos (tos, risas), el detector de tono del Autotune debe ignorar el ruido transitorio sin saltar erráticamente de octava.
- **Silencio prolongado:** Cuando el usuario calla, los efectos no deben amplificar el piso de ruido de la habitación (la puerta de ruido/VAD previa debe mantener el canal limpio).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema DEBE ofrecer un modo de corrección tonal (*Autotune*) capaz de cuantizar la frecuencia de la voz a la escala musical elegida (Cromática, Mayor, Menor o Pentatónica) con velocidad de corrección instantánea.
- **FR-002**: El sistema DEBE ofrecer un modo vocal Anime capaz de transponer el tono fundamental preservando y escalando la envolvente acústica de los formantes para simular un tracto vocal juvenil femenino.
- **FR-003**: El sistema DEBE incorporar un banco de efectos meme esenciales: Radio Táctica (paso banda con distorsión analógica y Roger Beep), Demonio (pitch grave con subarmónicos), Robot (modulación en anillo) y Catedral (reverberación de decaimiento largo).
- **FR-004**: El sistema DEBE incluir una función *Push-to-Bleep* que sustituya la señal vocal por una onda senoidal continua de 1.000 Hz mientras se mantenga activo el comando.
- **FR-005**: El sistema DEBE aplicar transiciones suaves (*crossfade* de 2 a 5 ms) entre cualquier conmutación de efecto o entrada/salida de censura para erradicar cualquier artefacto de click o saturación.
- **FR-006**: El sistema DEBE permitir controlar y conmutar todos los efectos tanto en modo permanente (*Toggle*) como en modo mientras se presiona (*Push-to-Effect*).
- **FR-007**: El sistema DEBE procesar los efectos en la computadora receptora para evitar recalentamiento y agotamiento de batería en el smartphone emisor.

### Key Entities

- **Efecto de Voz (Voice Effect):** Representa un perfil DSP activo con su tipo de algoritmo (Pitch, Formantes, Filtros, Modulación, Reverb), sus parámetros de intensidad y su estado de activación.
- **Escala Musical:** Colección de frecuencias objetivo válidas para la cuantización de notas en el motor de Autotune.
- **Comando de Efecto (FX Command):** Mensaje de control ligero que transmite la orden de cambio de preset o el estado del botón Bleep desde el controlador hacia el motor de procesamiento.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La latencia adicional introducida por el motor de efectos no supera los 6 milisegundos sobre el pipeline base de audio.
- **SC-002**: El 100% de las transiciones entre efectos se ejecutan sin clics ni discontinuidades audibles en la señal de audio.
- **SC-003**: En pruebas de inteligibilidad, al menos el 95% de las palabras habladas bajo el efecto Anime o Autotune son comprendidas claramente por oyentes sin entrenamiento.
- **SC-004**: El comando de Bleep de censura responde en menos de 2 milisegundos desde que se presiona el botón.
- **SC-005**: El consumo adicional de procesador en la PC debido a los efectos no supera el 1.5% en un procesador estándar de 4 núcleos.

## Assumptions

- El usuario habla con volumen suficiente para que el detector de frecuencia fundamental distinga los fonemas vocales del silencio.
- Los efectos se aplican en la PC después de cualquier etapa previa de eliminación de ruido de fondo para garantizar que los armónicos del efecto no sean distorsionados por ruido ambiental.
- El usuario dispone de audífonos para escuchar el resultado sin acople acústico de los altavoces al micrófono.
