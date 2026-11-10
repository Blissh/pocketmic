# Feature Specification: Android Audio Capture & Mobile Client

**Feature Branch**: `003-android-audio-capture`

**Created**: 2026-11-09

**Status**: Draft

**Input**: User description: "Android Audio Capture & Mobile Client: Captura acústica en smartphone con baja latencia, códec Opus, selector de cápsula física, sidetone local en audífonos y panel de control Stream Deck"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Captura acústica continua de alta fidelidad y baja latencia (Priority: P1)

Como usuario que utiliza su smartphone Android como micrófono para su computadora,  
quiero que el teléfono capture mi voz con fidelidad limpia (48 kHz) y latencia mínima sin interrupciones,  
para que mi voz suene nítida y en tiempo real en mis llamadas o transmisiones.

**Why this priority**: Es la función primaria del dispositivo móvil en el ecosistema de PocketMic.

**Independent Test**: Instalar la app en un teléfono Android, iniciar la captura y verificar que la señal de voz se adquiere continuamente a 48 kHz sin caídas de buffer ni saturación indebida.

**Acceptance Scenarios**:

1. **Given** que el usuario otorga permisos de micrófono y pulsa "Transmitir", **When** habla al teléfono, **Then** el motor de captura adquiere la voz de forma continua en fragmentos de 10 ms sin pausas audibles.
2. **Given** que la pantalla del smartphone se apaga o el usuario cambia a otra aplicación, **When** la transmisión está activa, **Then** el audio continúa capturándose en segundo plano sin interrupciones.

---

### User Story 2 - Emparejamiento y transmisión por red con fallback manual (Priority: P1)

Como usuario en una red doméstica o en una red pública con aislamiento (*AP Isolation*),  
quiero que la app encuentre mi PC automáticamente en un toque, y me permita escribir la IP manualmente si el router bloquea la búsqueda,  
para poder conectarme en cualquier escenario de red.

**Why this priority**: Evita la frustración del usuario en redes universitarias o con routers restrictivos.

**Independent Test**: Abrir la app en red normal (la PC aparece en pantalla en menos de 2 segundos) y en red restringida (botón de "Ingresar IP de la PC" establece la conexión en menos de 5 segundos).

**Acceptance Scenarios**:

1. **Given** que la PC y el móvil están en la misma red Wi-Fi, **When** la app se abre, **Then** detecta automáticamente el nombre de la PC y permite iniciar la transmisión con un toque.
2. **Given** que el router bloquea el auto-descubrimiento, **When** el usuario introduce manualmente la IP de su PC, **Then** la app establece la sesión de streaming de inmediato.

---

### User Story 3 - Selector de cápsula física de micrófono (Priority: P2)

Como usuario que coloca su teléfono en un soporte de escritorio o atril,  
quiero elegir si el audio se captura desde el micrófono inferior o el micrófono superior/frontal,  
para evitar que el sonido se escuche apagado u obstruido cuando el orificio inferior queda tapado contra la mesa.

**Why this priority**: Diferenciador acústico directo contra la competencia (Micstream solo usa el micrófono por defecto, que casi siempre queda tapado en un soporte).

**Independent Test**: Colocar el teléfono de pie sobre una mesa y alternar entre "Micrófono Inferior" y "Micrófono Frontal/Superior", verificando una ganancia de claridad inmediata al usar el micrófono orientado hacia el usuario.

**Acceptance Scenarios**:

1. **Given** que el teléfono está de pie sobre la mesa, **When** el usuario selecciona "Micrófono Superior", **Then** el teléfono conmuta la cápsula activa y la voz recupera todo su brillo y claridad.
2. **Given** que el usuario toma el teléfono en mano, **When** conmuta a "Micrófono Inferior", **Then** la captura se adapta al modo conversación natural.

---

### User Story 4 - Sidetone local en audífonos (Priority: P2)

Como usuario que tiene auriculares conectados al teléfono (por cable o Bluetooth),  
quiero escuchar mi propia voz en mis oídos sin ningún retraso mientras hablo,  
para no tener la sensación de "oído tapado" (*efecto oclusión*) al hablar en una llamada larga.

**Why this priority**: Mejora drásticamente la comodidad y reduce la fatiga vocal del usuario sin añadir carga a la red.

**Independent Test**: Conectar auriculares al móvil y hablar; el retorno de la propia voz debe escucharse de forma instantánea ($< 2$ ms).

**Acceptance Scenarios**:

1. **Given** que hay audífonos conectados al teléfono, **When** el usuario activa la opción "Monitoreo Local", **Then** escucha su propia voz en tiempo real con cero desfase.

---

### User Story 5 - Panel táctil tipo Stream Deck y VU Meter (Priority: P3)

Como streamer o participante en conferencias,  
quiero tener en la pantalla del móvil un medidor visual de nivel (VU Meter) con aviso de saturación (*clipping*), un botón para silenciar rápidamente y botones grandes para activar efectos (Autotune, Anime, Radio, Bleep),  
para controlar todo mi sonido con una mano sin tocar el teclado ni el ratón de la computadora.

**Why this priority**: Transforma el smartphone en un periférico físico interactivo de alta gama (*Stream Deck* de voz).

**Independent Test**: Tocar los botones de la pantalla del móvil y verificar que la PC cambia de efecto instantáneamente, observando el medidor de nivel responder al volumen de la voz.

**Acceptance Scenarios**:

1. **Given** que la app está transmitiendo, **When** el usuario pulsa un botón de efecto (ej. Anime o Autotune), **Then** la PC cambia el procesamiento en menos de 5 ms.
2. **Given** que el usuario mantiene pulsado el botón "BLEEP", **Then** la voz en la PC es censurada con un pitido hasta que se levanta el dedo.

---

### Edge Cases

- **Entrante de llamada telefónica celular:** El sistema operativo móvil debe pausar automáticamente la captura de PocketMic y reanudarla limpiamente al finalizar la llamada.
- **Desconexión de auriculares durante sidetone activo:** Si el usuario desenchufa los auriculares, el sidetone debe silenciarse instantáneamente para evitar acople o pitido por los altavoces del teléfono.
- **Pérdida de señal Wi-Fi:** Si el móvil se aleja del router, debe alertar visualmente al usuario y reconectar automáticamente en cuanto recupere señal.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La aplicación DEBE capturar audio a 48 kHz y 32-bit Float utilizando las APIs de menor latencia del sistema operativo Android.
- **FR-002**: La aplicación DEBE codificar la señal en frames de 10 ms utilizando el códec Opus optimizado para voz de baja latencia.
- **FR-003**: La aplicación DEBE implementar auto-descubrimiento en red local por UDP y permitir conexión manual por dirección IP.
- **FR-004**: La aplicación DEBE permitir alternar la cápsula de micrófono activa entre el micrófono inferior y el micrófono frontal/superior.
- **FR-005**: La aplicación DEBE ofrecer retorno local de voz (*Sidetone*) de latencia inmediata ($< 2$ ms) cuando se detecten audífonos conectados.
- **FR-006**: La aplicación DEBE ejecutarse como un servicio en primer plano (*Foreground Service*) con notificación persistente para garantizar captura continua con la pantalla apagada.
- **FR-007**: La aplicación DEBE incluir una cuadrícula de botones táctiles (*Stream Deck*) para conmutar los efectos de voz en la PC y un botón *Push-to-Bleep*.
- **FR-008**: La aplicación DEBE mostrar un medidor visual de amplitud (*VU Meter*) con alerta visual cuando la señal entre en rango de saturación digital (*clipping*).

### Key Entities

- **Perfil de Dispositivo Móvil:** Especificaciones del hardware del smartphone (cápsulas físicas disponibles, conectores de audio, capacidades de latencia).
- **Sesión de Streaming Móvil:** Estado actual del emisor (Desconectado, Buscando, Transmitiendo, Silenciado, Calidad de Red).
- **Control Pad (Stream Deck):** Colección de acciones táctiles y presets asignados a los botones de la pantalla móvil.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La latencia de captura interna en el smartphone es menor a 6 milisegundos.
- **SC-002**: El sidetone local en audífonos cableados tiene un retraso imperceptible menor a 2 milisegundos.
- **SC-003**: La transmisión de voz se mantiene 100% activa durante al menos 90 minutos continuos con la pantalla apagada sin ser detenida por el optimizador de batería del sistema.
- **SC-004**: La conmutación de cápsula física (inferior a superior) se ejecuta en menos de 50 milisegundos sin reiniciar la sesión de red.
- **SC-005**: El consumo de batería en el smartphone durante streaming continuo no supera el 5% por hora de uso.

## Assumptions

- El smartphone cuenta con Android 8.0 (API nivel 26) o superior para soporte de audio de baja latencia.
- El usuario concede el permiso de grabación de audio (`RECORD_AUDIO`) al abrir la aplicación.
- El dispositivo cuenta con conectividad Wi-Fi o enlace por cable USB a la computadora.
