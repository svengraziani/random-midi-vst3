# random_midi_vst

Ein simples MIDI-Generator-Plugin in Rust.

- Framework: **nice-plug** (Community Fork)
- GUI: **egui**
- Formate: **VST3** + **CLAP**
- Optional: **Standalone-App** für schnelle Entwicklung
- Kein Audio-I/O, nur MIDI-Output

## Features (MVP)

- Zufällige Noten (C minor pentatonic)
- Periodisches Triggern über Sample-Clock
- Probability-Gate
- Velocity-Min/Max
- Root Note + Octave Range
- NoteOff vor neuer NoteOn
- Einfache GUI mit Live-Status und Last Note

## Voraussetzungen

- Rust Toolchain (stable)

```bash
rustup update
rustc --version
cargo --version
```

## Build

Debug-Check:

```bash
cargo check
```

Release-Bundle (VST3 + CLAP):

```bash
cargo xtask bundle random_midi_vst --release
```

Ergebnis:

```text
target/bundled/random_midi_vst.vst3
target/bundled/random_midi_vst.clap
```

## Standalone starten

Für schnelles Testen ohne DAW:

```bash
cargo run --release
```

Nützliche Optionen anzeigen:

```bash
cargo run -- --help
```

Beispiel: explizit Dummy-Backend (ohne Audio-Device):

```bash
cargo run -- --backend dummy
```

## Installation

### macOS (VST3)

Nach:

```text
~/Library/Audio/Plug-Ins/VST3/
```

oder systemweit:

```text
/Library/Audio/Plug-Ins/VST3/
```

### Windows (VST3)

Nach:

```text
C:\Program Files\Common Files\VST3\
```

## Ableton Live Setup

1. MIDI Track erstellen
2. `random_midi_vst` auf den Track laden
3. Zweiten MIDI Track erstellen
4. Instrument (z. B. Synth) auf den zweiten Track laden
5. Bei Track 2: **MIDI From** = Track mit `random_midi_vst`
6. **Monitor = IN**

Danach sollte das Instrument die generierten Noten spielen.

## Aktuelle Parameter

- Enabled
- Rate
- Probability
- Root Note
- Octave Range
- Velocity Min
- Velocity Max

## Projektstruktur

```text
.
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── params.rs
│   └── editor.rs
├── xtask/
│   ├── Cargo.toml
│   └── src/main.rs
└── .cargo/config.toml
```
