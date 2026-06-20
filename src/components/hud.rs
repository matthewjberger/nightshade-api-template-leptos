use leptos::prelude::*;
use protocol::ClientMessage;
use wasm_bindgen::JsCast;

use crate::bridge::{Bridge, send};
use crate::state::TemplateState;

/// Example HUD panel: renderer facts streamed from the worker, a spin toggle,
/// and a button that sends a game message back. Replace with your own UI as
/// the game grows.
#[component]
pub fn Hud(
    bridge: StoredValue<Option<Bridge>, LocalStorage>,
    state: TemplateState,
) -> impl IntoView {
    let on_spawn = move |event: web_sys::MouseEvent| {
        if let Some(button) = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = button.blur();
        }
        if let Some(bridge) = bridge.get_value() {
            send(&bridge, &ClientMessage::SpawnCube);
        }
    };

    let on_spin = move |event: web_sys::Event| {
        let spinning = event_target_checked(&event);
        state.spinning.set(spinning);
        if let Some(bridge) = bridge.get_value() {
            send(&bridge, &ClientMessage::SetSpin { spinning });
        }
    };

    view! {
        <div class="hud">
            <div class="hud-title">"nightshade-api + leptos"</div>
            <div class="hud-row">
                <span class="hud-label">"Adapter"</span>
                <span>{move || state.adapter.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"FPS"</span>
                <span>{move || format!("{:.0}", state.fps.get())}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Entities"</span>
                <span>{move || state.entity_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Cubes"</span>
                <span>{move || state.cube_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Selected"</span>
                <span>
                    {move || {
                        state
                            .selected
                            .get()
                            .map(|detail| format!("{} ({})", detail.name, detail.id))
                            .unwrap_or_else(|| "None".to_string())
                    }}
                </span>
            </div>
            <button class="hud-button" on:click=on_spawn>
                "Spawn Cube (Space)"
            </button>
            <label class="hud-toggle">
                <input
                    type="checkbox"
                    prop:checked=move || state.spinning.get()
                    on:change=on_spin
                />
                "Spin"
            </label>
            <div class="hud-hint">"Drag to orbit, right-drag to pan, wheel to zoom, click to select"</div>
        </div>
    }
}
