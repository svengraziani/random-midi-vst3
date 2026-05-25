mod editor;
mod params;

use std::num::NonZeroU32;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use nice_plug::prelude::*;
use params::RandomMidiParams;

const SCALE_MINOR_PENTATONIC: [u8; 5] = [0, 3, 5, 7, 10];

#[derive(Default)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn seeded(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
        }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 32) as u32
    }

    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    fn range_u8(&mut self, min: u8, max_inclusive: u8) -> u8 {
        if min >= max_inclusive {
            return min;
        }

        let span = (max_inclusive - min) as u32 + 1;
        min + (self.next_u32() % span) as u8
    }

    fn choose<T: Copy>(&mut self, items: &[T]) -> T {
        let idx = (self.next_u32() as usize) % items.len();
        items[idx]
    }
}

pub struct RandomMidiVst {
    params: Arc<RandomMidiParams>,
    last_note: Arc<AtomicI32>,
    sample_rate: f32,
    sample_clock: u64,
    next_step_at: u64,
    active_note: Option<u8>,
    rng: XorShift64,
}

impl Default for RandomMidiVst {
    fn default() -> Self {
        Self {
            params: RandomMidiParams::new(),
            last_note: Arc::new(AtomicI32::new(-1)),
            sample_rate: 44_100.0,
            sample_clock: 0,
            next_step_at: 0,
            active_note: None,
            rng: XorShift64::seeded(0xDEADBEEFCAFEBABE),
        }
    }
}

impl RandomMidiVst {
    fn samples_per_step(&self) -> u64 {
        let rate_hz = self.params.rate.value();
        let rate_hz = rate_hz.max(0.05);
        (self.sample_rate / rate_hz).max(1.0) as u64
    }

    fn random_note(&mut self) -> u8 {
        let root = self.params.root_note.value() as i32;
        let octaves = self.params.octave_range.value().max(1) as u8;
        let degree = self.rng.choose(&SCALE_MINOR_PENTATONIC) as i32;
        let octave_offset = self.rng.range_u8(0, octaves.saturating_sub(1)) as i32;
        (root + degree + octave_offset * 12).clamp(0, 127) as u8
    }

    fn random_velocity(&mut self) -> f32 {
        let min_v = self.params.min_velocity.value().clamp(1, 127) as f32;
        let max_v = self.params.max_velocity.value().clamp(1, 127) as f32;
        let (lo, hi) = if min_v <= max_v { (min_v, max_v) } else { (max_v, min_v) };
        let midi_velocity = lo + self.rng.next_f32() * (hi - lo);
        midi_velocity / 127.0
    }

    fn trigger_step(&mut self, timing: u32, context: &mut impl ProcessContext<Self>) {
        if let Some(note) = self.active_note.take() {
            context.send_event(NoteEvent::NoteOff {
                timing,
                voice_id: None,
                channel: 0,
                note,
                velocity: 0.0,
            });
        }

        let probability = self.params.probability.value().clamp(0.0, 1.0);
        if self.rng.next_f32() > probability {
            self.last_note.store(-1, Ordering::Relaxed);
            return;
        }

        let note = self.random_note();
        let velocity = self.random_velocity();

        context.send_event(NoteEvent::NoteOn {
            timing,
            voice_id: None,
            channel: 0,
            note,
            velocity,
        });

        self.active_note = Some(note);
        self.last_note.store(note as i32, Ordering::Relaxed);
    }
}

impl Plugin for RandomMidiVst {
    type SysExMessage = ();
    type BackgroundTask = ();

    const NAME: &'static str = "Random MIDI VST";
    const VENDOR: &'static str = "Sven Graziani";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "hello@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        // Some hosts (including Ableton in certain setups) won't instantiate plugins with
        // completely empty audio layouts. We still behave as a MIDI generator, but expose a
        // regular stereo layout for compatibility.
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    // Ableton expects VST3 instruments to expose a valid event input bus.
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.last_note.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.sample_clock = 0;
        self.next_step_at = 0;
        self.active_note = None;
        true
    }

    fn reset(&mut self) {
        self.sample_clock = 0;
        self.next_step_at = 0;
        self.active_note = None;
        self.last_note.store(-1, Ordering::Relaxed);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if !self.params.enabled.value() {
            return ProcessStatus::KeepAlive;
        }

        let step_samples = self.samples_per_step().max(1);
        let num_samples = buffer.samples() as u64;

        for local_sample in 0..num_samples {
            if self.sample_clock >= self.next_step_at {
                self.trigger_step(local_sample as u32, context);
                self.next_step_at = self.sample_clock + step_samples;
            }
            self.sample_clock += 1;
        }

        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for RandomMidiVst {
    const CLAP_ID: &'static str = "com.svengraziani.random-midi-vst";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Random MIDI note generator");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for RandomMidiVst {
    // Bumped again so hosts won't reuse stale scan metadata.
    const VST3_CLASS_ID: [u8; 16] = *b"RandMidiVst00003";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nice_export_clap!(RandomMidiVst);
nice_export_vst3!(RandomMidiVst);
