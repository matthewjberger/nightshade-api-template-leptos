//! The wasm module inside the web worker. The engine `World`, the render
//! loop, and the page conversation (input injection, resize, picking, stats)
//! are all owned by `nightshade_api::offscreen::run_offscreen`; this crate is
//! only the game. The scene and the per-frame logic are written against
//! `nightshade-api`, the procedural high level facade: plain data in
//! `state.rs` and free functions in `systems/`, no user-side ECS.

mod state;
mod systems;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    nightshade_api::offscreen::run_offscreen(
        state::Scene::new(),
        systems::setup::initialize,
        systems::example::tick,
        systems::example::apply_custom,
    );
}
