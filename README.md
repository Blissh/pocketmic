# 🎙️ PocketMic

> **El micrófono de estudio que ya llevas en el bolsillo.**  
> Transforma tu smartphone (Android / iOS) en un micrófono físico externo de alta fidelidad para Windows y Linux, con cancelación de ruido por IA en tiempo real y calidad de audio sin pérdida.

---

## 📖 Visión General

**PocketMic** es una alternativa moderna, eficiente y orientada a la máxima calidad acústica frente a soluciones existentes como Micstream, AudioRelay y WO Mic.

A través de un controlador virtual en el sistema operativo y un motor de red de ultra-baja latencia sobre UDP, tu PC reconoce tu teléfono exactamente como si fuera un micrófono USB de estudio conectado físicamente, compatible de inmediato con:
* **Discord & TeamSpeak**
* **OBS Studio & Streamlabs**
* **Zoom, Microsoft Teams & Google Meet**
* **Videojuegos (In-Game Voice)**
* **Herramientas de Dictado y Modelos de Voz / IA (Whisper)**

---

## 🚀 Documentación y Arquitectura del Proyecto

Este repositorio contiene la investigación de mercado, ingeniería inversa competitiva y especificaciones técnicas completas:

1. **[Análisis Integral de Micstream (Benchmark y Reverse Engineering)](ANALISIS_COMPLETO_MICSTREAM.md):**
   * Desglose técnico de códecs (Opus, 32-bit float, 48 kHz).
   * Modelo de Inteligencia Artificial (DeepFilterNet 3 ONNX en PC).
   * Estrategias de monetización (Rewarded Ads, RevenueCat, suscripciones).
   * Matriz comparativa vs AudioRelay y WO Mic.
2. **[Propuestas de Mejora y Arquitectura Lean de PocketMic](PROPUESTAS_Y_ARQUITECTURA_OPENMIC.md):**
   * *Studio Voice Strip* nativa integrada (High-Pass 80Hz, Leveler y EQ de 3 bandas).
   * *Streamer Fun FX & Soundboard táctil* (Autotune en tiempo real, timbre Anime/VTuber con formantes, Ardilla, Radio táctica y Bleep de censura).
   * Sidetone local con audífonos (monitoreo en tiempo real con latencia cero).
   * Canal dúplex opcional de audio inverso (PC a smartphone).
   * Selector de cápsula física de micrófono (evita obstrucción en atril/mesa).
   * Resiliencia de red con fallback manual por IP / PIN para sortear *AP Isolation*.
   * Conexión USB Plug & Play en Android (AOA) e iOS (túnel `usbmuxd`).
   * Controles de hardware: *Flip-to-Mute* táctil y Push-to-Talk háptico.

---

## 🛠️ Estructura del Repositorio

```text
pocketmic/
├── ANALISIS_COMPLETO_MICSTREAM.md      # Análisis competitivo y de ingeniería inversa
├── PROPUESTAS_Y_ARQUITECTURA_OPENMIC.md # Arquitectura técnica de PocketMic y roadmap
├── scrape_all_md.py                    # Script de scraping profundo (Scrapling)
├── scrape_micstream.py                 # Extractor de datos JSON y HTML
├── scraped_data/                       # Datos crudos extraídos de la web oficial
├── scraped_md/                         # Páginas, guías y políticas en Markdown limpio
└── README.md                           # Documentación principal
```

---

## 📋 Hoja de Ruta (Roadmap Resumido)

- [ ] **Fase 1 (MVP Core):** Captura en móvil (Google Oboe C++ / CoreAudio) + Enlace UDP Opus + Driver virtual WASAPI/PipeWire.
- [ ] **Fase 2 (Calidad, FX y UX):** Integración DeepFilterNet 3 + Voice Strip DSP + Streamer Fun FX (Autotune, Anime, Meme) + Sidetone local + Flip-to-Mute.
- [ ] **Fase 3 (Conectividad Avanzada):** Modo Android AOA + Túnel USB en iOS + Canal Dúplex de audio inverso.

---

## 📄 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE).
