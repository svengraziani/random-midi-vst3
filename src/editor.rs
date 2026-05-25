use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use egui::RichText;
use nice_plug::prelude::*;
use nice_plug_egui::{create_egui_editor, widgets, EguiState};

use crate::params::RandomMidiParams;

pub fn create(params: Arc<RandomMidiParams>, last_note: Arc<AtomicI32>) -> Option<Box<dyn Editor>> {
    let editor_state: Arc<EguiState> = params.editor_state.clone();

    create_egui_editor(
        editor_state,
        (),
        Default::default(),
        |_egui_ctx, _queue, _state| {},
        move |ui, setter, _queue, _state| {
            ui.heading("Random MIDI VST");
            ui.separator();

            let status = if params.enabled.value() { "RUNNING" } else { "STOPPED" };
            ui.label(RichText::new(format!("Status: {status}")).strong());

            let current_note = last_note.load(Ordering::Relaxed);
            if current_note >= 0 {
                ui.label(format!("Last Note: {}", note_name(current_note)));
            } else {
                ui.label("Last Note: —");
            }

            ui.separator();

            let mut enabled = params.enabled.value();
            if ui.checkbox(&mut enabled, "Enabled").changed() {
                setter.begin_set_parameter(&params.enabled);
                setter.set_parameter(&params.enabled, enabled);
                setter.end_set_parameter(&params.enabled);
            }

            ui.add(widgets::ParamSlider::for_param(&params.rate, setter));
            ui.add(widgets::ParamSlider::for_param(&params.probability, setter));
            ui.add(widgets::ParamSlider::for_param(&params.root_note, setter));
            ui.add(widgets::ParamSlider::for_param(&params.octave_range, setter));
            ui.add(widgets::ParamSlider::for_param(&params.min_velocity, setter));
            ui.add(widgets::ParamSlider::for_param(&params.max_velocity, setter));
        },
    )
}

fn note_name(note: i32) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let clamped = note.clamp(0, 127);
    let name = NAMES[(clamped % 12) as usize];
    let octave = (clamped / 12) - 1;
    format!("{}{}", name, octave)
}
