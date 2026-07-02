use leptos::prelude::*;
use nightshade_api::web::{EngineViewport, Loader, UiStyles, WebGpuGate, use_engine};
use protocol::Event;

use crate::components::hud::Hud;
use crate::state::TemplateState;

/// Application root: creates the engine handle (which spawns the worker and
/// forwards keyboard input), routes game events into the page state, and
/// composes the viewport and overlays. `WebGpuGate` falls back to a notice
/// when the browser has no WebGPU.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <UiStyles />
        <WebGpuGate>
            <Stage />
        </WebGpuGate>
    }
}

#[component]
fn Stage() -> impl IntoView {
    let engine = use_engine("runtime/worker.js");
    let state = TemplateState::new();

    engine.on_custom(Callback::new(move |value: serde_json::Value| {
        if let Ok(Event::CubeCount { count }) = serde_json::from_value(value) {
            state.cube_count.set(count);
        }
    }));

    view! {
        <div class="app-shell">
            <EngineViewport engine=engine />
            <Hud engine=engine state=state />
            <Loader ready=engine.state.ready />
        </div>
    }
}
