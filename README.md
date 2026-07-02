# template_api_leptos

A [nightshade-api](https://crates.io/crates/nightshade-api) app with a [Leptos](https://leptos.dev) frontend. The engine runs in a web worker on an `OffscreenCanvas`, off the browser's main thread, and the scene and per-frame logic are written against the `nightshade-api` facade: plain data and free functions, no user-side ECS. A Leptos UI on the main thread overlays it. The worker seam (canvas transfer, input forwarding, picking, resize, stats) is `nightshade-api` itself: the `leptos` feature on the page side, the `offscreen` feature on the worker side. Copy this directory to start a project.

This is the same architecture as `template_leptos`, with one difference: the worker's game logic is written against `nightshade-api` instead of the raw `nightshade` engine. Compare the two `worker/` crates to see what the facade buys you.

## Run

```sh
just run       # native webview window over the web bundle
just run-web   # serve in the browser via trunk
```

`just init` installs the pinned toolchain (rust, wasm-bindgen, wasm-opt, trunk) through mise. Both paths render through WebGPU, so use a browser or platform webview with WebGPU and OffscreenCanvas-in-workers support (Chromium 113+, Firefox 141+).

## How it works

Two threads, one seam. The page never touches the engine and the worker never touches the DOM. The transport (input, resize, picking, stats) is built into `nightshade-api`; only game messages are yours, carried as `Custom` payloads.

- `protocol/` is the game wire format: the `Command` (page to worker) and `Event` (worker to page) enums, shared by both sides.
- `worker/` is the game. `src/lib.rs` hands `nightshade_api::offscreen::run_offscreen` the scene, a setup function, a per-frame tick, and a `Command` handler. `src/state.rs` holds the plain `Scene` data, and `src/systems/` is the game logic as straight-line `nightshade-api` calls.
- `src/` is the page. `app.rs` creates the engine handle with `use_engine` and composes `EngineViewport`, the HUD, and the loader from `nightshade_api::web`; `state.rs` is the game-specific page state as `Copy` signals; renderer facts (ready, adapter, FPS, entities, selection) arrive on the handle's reactive `EngineState`.

Grow it by adding a `protocol` variant, sending it with `engine.send`, handling it in `apply_custom` (`worker/src/systems/example.rs`), and building the UI under `components/`. The `Paint Selected` button shows the round trip: the driver's built-in click pick reports the selection to the page and hands it to `apply_custom` on the worker.

## Desktop

`desktop/` is a native shell: it serves the built `dist/` on an ephemeral localhost port and opens it in a `wry` webview window. `just run` builds the bundle and launches it. Debug builds read `../dist` from disk; release builds embed it into the executable (`just build-desktop`).

## License

Dual-licensed under MIT or Apache-2.0, at your option.
