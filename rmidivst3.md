# Random MIDI VST3 Plugin – Bauplan

## Ziel

Ein simples VST3-Plugin für Ableton Live.

Das Plugin:
- erzeugt zufällige MIDI-Noten
- besitzt eine einfache GUI
- wird in Rust geschrieben
- nutzt nice-plug (Community Fork)
- dient als Basis für spätere AI-Features

Kein Audio.
Nur MIDI.

---

# Gesamtarchitektur

```text
Ableton MIDI Track
    ↓
Random MIDI VST3
    ↓
anderes Instrument
    ↓
Audio Output
```

Das Plugin erzeugt selbst keinen Klang.
Es sendet nur MIDI-Events.

---

# Technologie-Stack

## Core

- Rust
- nice-plug
- VST3
- egui

## Später für AI

- PyTorch
- ONNX
- Candle oder tract

---

# Warum Rust?

Rust eignet sich besser für:

- Realtime Audio
- sichere Speicherverwaltung
- Plugin-Stabilität
- Crossplattform
- langfristige Wartbarkeit
- MIDI/Event-Systeme
- spätere AI-Integration

---

# Projektstruktur

```text
random_midi_vst/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── params.rs
│   └── editor.rs
└── xtask/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

# Installation

## Rust

```bash
rustup update
rustc --version
cargo --version
```

---

# nice-plug Setup

```bash
cargo new random_midi_vst --lib
```

Dann in `Cargo.toml`:

```toml
[dependencies]
nice_plug = { package = "nice-plug", git = "https://codeberg.org/BillyDM/nice-plug.git", features = ["vst3"] }
nice_plug_egui = { package = "nice-plug-egui", git = "https://codeberg.org/BillyDM/nice-plug.git" }
```

Für das Bundling zusätzlich:

```toml
# xtask/Cargo.toml
[dependencies]
nice_plug_xtask = { package = "nice-plug-xtask", git = "https://codeberg.org/BillyDM/nice-plug.git" }
```

---

# Plugin-Typ

Das Plugin hat:

- keine Audio-Eingänge
- keine Audio-Ausgänge
- nur MIDI-Output

Konfiguration:

```rust
const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: None,
    main_output_channels: None,
    aux_input_ports: &[],
    aux_output_ports: &[],
    names: PortNames::const_default(),
}];
```

---

# MVP-Funktionalität

## Verhalten

Das Plugin:

- zählt Samples
- erzeugt periodisch neue MIDI-Noten
- sendet NoteOn/NoteOff
- nutzt Zufallswerte

---

# MIDI-Algorithmus

```text
pro Audio-Block:
    sample_position erhöhen

    wenn Step-Grenze erreicht:
        falls Note aktiv:
            NoteOff senden

        Zufallsnote wählen
        Velocity wählen
        NoteOn senden
```

---

# Erste musikalische Defaults

```text
Rate: alle 0.25 Sekunden
Scale: C minor pentatonic
Notenbereich: C3 bis C5
Velocity: 60 bis 110
Probability: 70 %
```

---

# Beispiel-Skala

```rust
const SCALE: [u8; 5] = [0, 3, 5, 7, 10];
```

---

# Notenberechnung

```text
root = 60
scale_degree = random from SCALE
octave_offset = random octave
note = root + scale_degree + octave_offset * 12
```

---

# GUI

## Ziel

Keine schöne GUI.
Nur funktional.

---

# GUI-Technologie

```text
nice-plug + egui
```

Warum:

- einfach
- schnell
- ausreichend für MVP
- keine native GUI-Hölle

---

# GUI Layout

```text
┌──────────────────────────────┐
│ Random MIDI VST              │
│                              │
│ Status: RUNNING              │
│ Last Note: C#4               │
│                              │
│ Rate:        [ 1/8      ]    │
│ Probability: [ 72%      ]    │
│ Root:        [ C        ]    │
│ Scale:       [ Minor    ]    │
│ Octaves:     [ 2        ]    │
│ Velocity:    [ 60–110   ]    │
│ Chaos:       [ 35%      ]    │
│                              │
│ [ Start / Stop ]             │
└──────────────────────────────┘
```

---

# Parameter

```rust
pub struct RandomMidiParams {
    pub enabled: BoolParam,
    pub rate: FloatParam,
    pub probability: FloatParam,
    pub root_note: IntParam,
    pub octave_range: IntParam,
    pub min_velocity: IntParam,
    pub max_velocity: IntParam,
    pub chaos: FloatParam,
}
```

---

# Dateien

## lib.rs

Enthält:

- Plugin-Definition
- MIDI-Engine
- Parameterzugriff
- Eventhandling

---

## params.rs

Enthält:

- Plugin-Parameter
- Default-Werte
- Automation

---

## editor.rs

Enthält:

- egui UI
- Slider
- Buttons
- Statusanzeigen

---

# Wichtig

Alle GUI-Werte müssen echte Plugin-Parameter sein.

Richtig:

```text
GUI → Parameter → Audio/MIDI Engine
```

Falsch:

```text
GUI → lokale GUI Variable
```

---

# Erste sinnvolle Controls

Version 0.1:

- Enabled
- Rate
- Probability
- Root Note
- Octave Range
- Velocity Min
- Velocity Max

---

# Noch NICHT bauen

Nicht für MVP:

- AI
- Preset Browser
- Piano Roll
- Pattern Editor
- Drag & Drop MIDI
- Cloud Features
- Neural Synthese

---

# Build

```bash
cargo xtask bundle random_midi_vst --release
```

---

# Ergebnis

```text
target/bundled/Random MIDI VST.vst3
```

---

# Installation

## macOS

```text
~/Library/Audio/Plug-Ins/VST3/
```

oder:

```text
/Library/Audio/Plug-Ins/VST3/
```

## Windows

```text
C:\Program Files\Common Files\VST3\
```

---

# Ableton

```text
Ableton → Settings → Plug-ins → Rescan
```

---

# Ableton Routing

1. MIDI Track erstellen
2. Random MIDI VST laden
3. Zweiten MIDI Track erstellen
4. Instrument laden
5. MIDI From setzen
6. Monitor = IN

---

# Realistische Reihenfolge

```text
1. Plugin lädt in Ableton
2. GUI sichtbar
3. MIDI Output funktioniert
4. Parameter ändern Verhalten
5. BPM Sync
6. bessere musikalische Regeln
7. Scale System
8. Presets
9. AI-Features
```

---

# Phase 2 Ideen

- BPM Sync
- Swing
- Euclidean Rhythm
- Chaos Parameter
- Probability pro Step
- Root/Scale Auswahl
- Seed Lock
- MIDI CC Output
- generative Sequenzen

---

# Spätere AI-Richtung

```text
Random MIDI Generator
    ↓
AI Pattern Model
    ↓
Neural Sequencer
    ↓
Neural Oscillator
```

---

# Neural Oscillator Idee

Ein neuronales Modell ersetzt klassische Oszillatoren.

Nicht:
- Saw
- Square
- FM

Sondern:
- latent spaces
- neural timbres
- morphing Klangräume

---

# Neural Architektur

```text
Python Training
    ↓
PyTorch
    ↓
ONNX Export
    ↓
Rust Runtime
    ↓
VST3 Plugin
```

---

# Wichtigstes Problem

Realtime Audio.

Du hast oft nur:

```text
1–3 ms
```

pro Buffer.

Deshalb:

- kleine Modelle
- quantisierte Weights
- keine riesigen Diffusionsmodelle
- keine blockierenden Operationen
- keine Allocations im Audio-Thread

---

# Beste langfristige Strategie

Nicht:

```text
AI erzeugt komplettes Audio
```

Sondern:

```text
AI erzeugt Klangstruktur
DSP rendert stabilen finalen Sound
```

---

# Empfohlene Entwicklungsstrategie

## Phase 1

- Random MIDI
- einfache GUI
- stabile Ableton Integration

## Phase 2

- musikalische Regeln
- generative Sequenzen
- bessere Timing-Engine

## Phase 3

- AI Pattern Generation
- kleine neuronale Modelle

## Phase 4

- Neural Oscillator
- latent morphing
- evolving textures

## Phase 5

- professionelles Produkt
- Preset Ecosystem
- Crossplattform Release

---

# Entscheidung

Für den Start:

```text
Rust + nice-plug + egui + VST3
```

Nicht:

- Zig
- JUCE
- eigener DSP-Framework-Wahnsinn
- Full-AI direkt am Anfang

Erst:

```text
funktionierendes hässliches Plugin
```

Dann:

```text
musikalisch gutes Plugin
```

Dann:

```text
AI-System
```

