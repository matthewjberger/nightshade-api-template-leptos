//! The Leptos UI on the main thread.
//!
//! ## Architecture
//!
//! The engine runs in a web worker on an `OffscreenCanvas`, written against
//! `nightshade-api`. The worker seam (canvas transfer, input forwarding,
//! picking, stats) is `nightshade_api::web` on this side and
//! `nightshade_api::offscreen` on the worker side, so this crate is only the
//! page: the game state, the HUD, and the game messages.
//!
//! - `src/app.rs` creates the engine handle and composes the components.
//! - `src/state.rs` is the game-specific page state, grouped as `Copy` signals.
//! - `src/components/` holds the components: the example HUD.
//!
//! Add a new feature by extending the `protocol` enums, sending them with
//! `engine.send` (page side), handling them in `apply_custom`
//! (`worker/src/systems/example.rs`), and building the UI in a new file under
//! `src/components/`.

mod app;
mod components;
mod state;

pub use app::App;
