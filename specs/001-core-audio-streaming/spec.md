# Feature Specification: Core Audio Streaming

**Feature Branch**: `001-core-audio-streaming`

**Created**: 2026-11-09

**Status**: Draft

**Input**: User description: "Core Audio Streaming: Captura en smartphone, transmisión UDP de baja latencia y decodificación Opus en PC"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Transmisión de voz inalámbrica en tiempo real (Priority: P1)

Como creador de contenido, jugador o participante en videollamadas,  
quiero hablar a través de mi smartphone y que mi computadora reciba mi voz con calidad nítida y latencia imperceptible,  
para poder usar mi teléfono como micrófono principal sin cables ni retrasos audibles en mis conversaciones.

**Why this priority**: Es el núcleo esencial y la propuesta de valor fundamental de PocketMic. Sin una transmisión de voz continua y de baja latencia, el producto no cumple su propósito principal.

**Independent Test**: Se puede probar de forma totalmente autónoma conectando el teléfono y la PC a la misma red Wi-Fi; al hablar en el teléfono, el sonido se reproduce en los auriculares o salida del PC en tiempo real con fidelidad acústica completa.

**Acceptance Scenarios**:

1. **Given** que la app móvil y el cliente de PC están emparejados, **When** el usuario habla al teléfono a volumen conversacional normal, **Then** el audio se escucha en la PC con fidelidad natural y un retardo imperceptible para la conversación fluida.
2. **Given** que la transmisión está activa, **When** el usuario habla de forma ininterrumpida durante 10 minutos, **Then** no se perciben distorsiones acumulativas, desfases de tiempo crecientes (*drift*) ni chasquidos en el sonido.

---

### User Story 2 - Descubrimiento automático con alternativa de conexión manual (Priority: P2)

Como usuario que se conecta en diferentes entornos (casa, universidad u oficina),  
quiero que la app móvil encuentre mi computadora automáticamente al abrirse, y que me ofrezca ingresar manualmente la dirección de red si el router bloquea la búsqueda automática,  
para no quedar incomunicado en redes con aislamiento de clientes (*AP Isolation*).

**Why this priority**: Es el principal punto de fricción de la competencia. Si el usuario no puede conectar sus dispositivos de forma rápida y confiable, la aplicación queda inútil.

**Independent Test**: Se valida abriendo la app en una red estándar (emparejamiento instantáneo en menos de 2 segundos) y posteriormente en una red con bloqueo de difusión (*broadcast*), ingresando la dirección de la PC manualmente para establecer la sesión.

**Acceptance Scenarios**:

1. **Given** que la PC y el móvil están en la misma red Wi-Fi estándar, **When** el usuario abre la aplicación móvil, **Then** la PC aparece listada automáticamente en pantalla lista para conectar en un solo toque.
2. **Given** que la red Wi-Fi bloquea el descubrimiento automático, **When** pasan 3 segundos sin detectar la PC, **Then** la aplicación ofrece la opción de ingresar la dirección IP de la computadora y conecta exitosamente al ingresarla.

---

### User Story 3 - Monitoreo visual de señal y silenciado rápido (Priority: P3)

Como usuario durante una transmisión en vivo o reunión de trabajo,  
quiero ver en la pantalla de mi teléfono un medidor del volumen de mi voz y un botón para silenciar el micrófono al instante,  
para saber si mi voz está saturando y poder toser o hablar en privado sin transmitir a la PC.

**Why this priority**: Brinda retroalimentación visual inmediata y control de privacidad esencial sin necesidad de tocar la computadora.

**Independent Test**: Se valida hablando al micrófono observando el medidor de nivel en pantalla, provocando un volumen alto para verificar la alerta de saturación, y pulsando el botón de silencio para confirmar que el audio se interrumpe de inmediato en la PC.

**Acceptance Scenarios**:

1. **Given** que el usuario está transmitiendo, **When** el volumen de su voz sube o baja, **Then** la barra de nivel visual responde instantáneamente reflejando la intensidad.
2. **Given** que el usuario habla demasiado cerca o grita, **When** la señal entra en zona de distorsión (*clipping*), **Then** el medidor cambia a color rojo alertando al usuario de inmediato.
3. **Given** que la transmisión está activa, **When** el usuario pulsa el botón de silenciar (*Mute*), **Then** la PC deja de recibir audio de inmediato y la pantalla muestra claramente el estado de silenciado.

---

### Edge Cases

- **Pérdida temporal de paquetes Wi-Fi:** Si el router experimenta interferencia transitoria (hasta 5% de paquetes perdidos), el sistema debe reconstruir la forma de onda de la voz de forma transparente sin generar ruidos metálicos estridentes ni desbordar la memoria.
- **Interrupción o bloqueo de pantalla en el móvil:** Si la pantalla del smartphone se apaga o el usuario cambia de aplicación, la transmisión de voz debe continuar en segundo plano sin cortes.
- **Entrante de llamada telefónica celular:** Si entra una llamada tradicional al teléfono, el sistema operativo debe priorizar la llamada y pausar la transmisión de PocketMic elegantemente.
- **Desconexión o cambio de red:** Si el teléfono pierde la conexión Wi-Fi momentáneamente, debe pausar el flujo e intentar reconectarse automáticamente durante 10 segundos antes de pasar a estado desconectado con un aviso comprensible.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema DEBE capturar continuamente la señal acústica del micrófono del smartphone con frecuencia de 48 kHz y precisión de coma flotante de 32 bits.
- **FR-002**: El sistema DEBE comprimir y transmitir el flujo de audio mediante paquetes de red de baja latencia con fragmentos de 10 ms.
- **FR-003**: El sistema DEBE implementar descubrimiento automático entre el cliente móvil y el servidor de escritorio cuando ambos se encuentran en la misma subred.
- **FR-004**: El sistema DEBE proporcionar una opción de conexión directa manual por dirección de red para garantizar operatividad cuando el descubrimiento automático esté bloqueado por el router.
- **FR-005**: El sistema DEBE decodificar el flujo de audio recibido en la computadora y entregarlo a la capa de reproducción de audio del sistema operativo sin introducir retardos mayores al presupuesto establecido.
- **FR-006**: La aplicación móvil DEBE calcular y mostrar en tiempo real la amplitud de la señal (*VU Meter*) con indicación visual diferenciada para niveles óptimos y saturación (*clipping*).
- **FR-007**: El sistema DEBE permitir silenciar (*Mute*) el micrófono de forma instantánea tanto desde la pantalla del teléfono como desde el host de la computadora.
- **FR-008**: El sistema DEBE recuperarse de manera autónoma ante micro-cortes de red de hasta 3 segundos sin requerir que el usuario reinicie ninguna de las aplicaciones.

### Key Entities

- **Dispositivo Emisor (Móvil):** Representa el smartphone del usuario que captura la señal física, ejecuta la medición de nivel y despacha los paquetes de voz.
- **Host Receptor (PC):** Representa la computadora donde se reciben, reordenan y decodifican los paquetes de audio para ser inyectados al sistema de sonido.
- **Sesión de Audio:** Entidad que encapsula el estado de la comunicación (Desconectado, Buscando, Conectado, Silenciado), el canal de transporte activo y las métricas de calidad (pérdida de paquetes, fluctuación/jitter y tiempo de conexión).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El retardo total de la voz desde que ingresa al micrófono del teléfono hasta que está disponible en la PC es menor a 15 milisegundos en condiciones normales de red inalámbrica (Wi-Fi 5 GHz).
- **SC-002**: El 95% de los emparejamientos en redes locales domésticas se concreta automáticamente en menos de 3 segundos tras abrir la aplicación.
- **SC-003**: En entornos con bloqueo de difusión, el usuario logra conectarse exitosamente mediante ingreso manual de dirección en menos de 10 segundos.
- **SC-004**: El sistema mantiene transmisión continua y estable durante sesiones de al menos 60 minutos sin pérdida acumulativa de sincronía (*drift* de audio menor a 5 ms por hora).
- **SC-005**: Tras una interrupción abrupta de red de 2 segundos, el flujo de audio se reanuda de forma audiblemente normal en menos de 1.5 segundos tras restaurarse el enlace.
- **SC-006**: El consumo de procesador en la computadora durante la recepción y decodificación no supera el 2.0% en un equipo de escritorio estándar.

## Assumptions

- El smartphone y la computadora están conectados a la misma red de área local o enlazados directamente.
- El usuario ha otorgado permisos de acceso al micrófono en el sistema operativo del teléfono.
- El dispositivo receptor en la computadora cuenta con un subsistema de audio funcional (WASAPI en Windows o PipeWire en Linux).
- El ancho de banda disponible en la red local es de al menos 200 kbps simétricos dedicados para el flujo de voz.
