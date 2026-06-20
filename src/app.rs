use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use protocol::ClientMessage;

use crate::bridge::{Bridge, send};
use crate::components::hud::Hud;
use crate::components::loader::Loader;
use crate::components::viewport::Viewport;
use crate::state::TemplateState;

/// Application root: owns the shared state and bridge slot, forwards keyboard
/// input to the worker, and composes the viewport and overlays. Falls back to
/// a notice when the browser has no WebGPU.
#[component]
pub fn App() -> impl IntoView {
    if !webgpu_supported() {
        return unsupported().into_any();
    }

    let state = TemplateState::new();
    let bridge = StoredValue::new_local(None::<Bridge>);

    let _ = window_event_listener(leptos::ev::keydown, move |event| {
        if typing_in_field(&event) {
            return;
        }
        if let Some(bridge) = bridge.get_value() {
            let text = (event.key().chars().count() == 1).then(|| event.key());
            send(
                &bridge,
                &ClientMessage::Key {
                    code: event.code(),
                    pressed: true,
                    text,
                },
            );
        }
    });

    let _ = window_event_listener(leptos::ev::keyup, move |event| {
        if typing_in_field(&event) {
            return;
        }
        if let Some(bridge) = bridge.get_value() {
            send(
                &bridge,
                &ClientMessage::Key {
                    code: event.code(),
                    pressed: false,
                    text: None,
                },
            );
        }
    });

    view! {
        <div class="app-shell">
            <Viewport bridge state />
            <Hud bridge state />
            <Loader state />
        </div>
    }
    .into_any()
}

fn unsupported() -> impl IntoView {
    view! {
        <div class="unsupported">
            <div class="unsupported-card">
                <h1>"WebGPU not available"</h1>
                <p>
                    "This app runs the Nightshade engine in a web worker through WebGPU. Open it in a browser with WebGPU and OffscreenCanvas-in-workers support (Chromium 113+, Firefox 141+)."
                </p>
            </div>
        </div>
    }
}

fn typing_in_field(event: &web_sys::KeyboardEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        .map(|element| {
            let tag = element.tag_name();
            tag.eq_ignore_ascii_case("input")
                || tag.eq_ignore_ascii_case("textarea")
                || tag.eq_ignore_ascii_case("select")
                || element.is_content_editable()
        })
        .unwrap_or(false)
}

fn webgpu_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("navigator"))
    else {
        return false;
    };
    js_sys::Reflect::get(&navigator, &JsValue::from_str("gpu"))
        .map(|gpu| !gpu.is_undefined() && !gpu.is_null())
        .unwrap_or(false)
}
