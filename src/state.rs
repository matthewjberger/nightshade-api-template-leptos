use leptos::prelude::*;
use protocol::SelectedEntity;

/// All page state, grouped as signals. `Copy`, so it threads into every
/// component and closure without cloning.
#[derive(Clone, Copy)]
pub struct TemplateState {
    pub ready: RwSignal<bool>,
    pub adapter: RwSignal<String>,
    pub fps: RwSignal<f32>,
    pub entity_count: RwSignal<u32>,
    pub cube_count: RwSignal<u32>,
    pub selected: RwSignal<Option<SelectedEntity>>,
    pub spinning: RwSignal<bool>,
    pub grabbing: RwSignal<bool>,
}

impl TemplateState {
    pub fn new() -> Self {
        Self {
            ready: RwSignal::new(false),
            adapter: RwSignal::new(String::new()),
            fps: RwSignal::new(0.0),
            entity_count: RwSignal::new(0),
            cube_count: RwSignal::new(0),
            selected: RwSignal::new(None),
            spinning: RwSignal::new(true),
            grabbing: RwSignal::new(false),
        }
    }
}

impl Default for TemplateState {
    fn default() -> Self {
        Self::new()
    }
}
