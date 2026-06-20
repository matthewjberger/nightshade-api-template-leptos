//! The Leptos UI on the main thread.
//!
//! ## Architecture
//!
//! The engine runs in a web worker on an `OffscreenCanvas`, written against
//! `nightshade-api`. This crate is only the page: the UI and the worker seam.
//!
//! - `src/app.rs` composes the components and forwards keyboard input.
//! - `src/bridge.rs` spawns the worker and converts `WorkerMessage`s into
//!   signal writes, and `ClientMessage`s into `postMessage` envelopes.
//! - `src/state.rs` is all page state, grouped as `Copy` signals.
//! - `src/components/` holds the components: the viewport canvas and the
//!   example HUD.
//!
//! Add a new feature by extending the `protocol` messages, handling them in
//! `bridge.rs` (page side) and `worker/src/lib.rs` (worker side), and
//! building the UI in a new file under `src/components/`.

mod app;
mod bridge;
mod components;
mod state;

pub use app::App;
