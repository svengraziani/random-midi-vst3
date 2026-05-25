use std::sync::Arc;

use nice_plug::prelude::*;
use nice_plug_egui::EguiState;

#[derive(Params)]
pub struct RandomMidiParams {
    #[id = "enabled"]
    pub enabled: BoolParam,

    #[id = "rate"]
    pub rate: FloatParam,

    #[id = "probability"]
    pub probability: FloatParam,

    #[id = "root_note"]
    pub root_note: IntParam,

    #[id = "octave_range"]
    pub octave_range: IntParam,

    #[id = "min_velocity"]
    pub min_velocity: IntParam,

    #[id = "max_velocity"]
    pub max_velocity: IntParam,

    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
}

impl RandomMidiParams {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            enabled: BoolParam::new("Enabled", true),
            rate: FloatParam::new(
                "Rate",
                4.0,
                FloatRange::Linear {
                    min: 0.25,
                    max: 16.0,
                },
            )
            .with_unit(" Hz")
            .with_smoother(SmoothingStyle::Linear(10.0)),
            probability: FloatParam::new(
                "Probability",
                0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit(" %")
            .with_value_to_string(Arc::new(|v| format!("{:.0}", v * 100.0)))
            .with_string_to_value(Arc::new(|s| s.parse::<f32>().ok().map(|v| v / 100.0))),
            root_note: IntParam::new("Root", 48, IntRange::Linear { min: 24, max: 72 })
                .with_value_to_string(Arc::new(format_note_name)),
            octave_range: IntParam::new("Octaves", 2, IntRange::Linear { min: 1, max: 4 }),
            min_velocity: IntParam::new("Vel Min", 60, IntRange::Linear { min: 1, max: 127 }),
            max_velocity: IntParam::new("Vel Max", 110, IntRange::Linear { min: 1, max: 127 }),
            editor_state: EguiState::from_size(360, 300),
        })
    }
}

fn format_note_name(note: i32) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let clamped = note.clamp(0, 127);
    let name = NAMES[(clamped % 12) as usize];
    let octave = (clamped / 12) - 1;
    format!("{}{}", name, octave)
}
