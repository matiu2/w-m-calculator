use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

use crate::output::Output;

pub mod output;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let mut pip_places = use_persistent("pip_places", || 4);
    let mut spread = use_persistent("spread", || 1.5);
    let high_low = use_persistent("high_low", || -> Option<f64> { None });
    let neckline = use_persistent("neckline", || -> Option<f64> { None });

    let get_input = |mut signal: Signal<Option<f64>>| {
        move |e: Event<FormData>| {
            let raw = e.value();
            let input = raw.trim();
            let value = if input.is_empty() {
                None
            } else {
                input.parse().ok()
            };
            signal.set(value);
        }
    };
    rsx! {
        div {
            id: "hero",
            class: "calculator-card",
            div {
                class: "field",
                label {
                    for: "spread",
                    "Broker spread"
                }
                input {
                    id: "spread",
                    type: "number",
                    value: spread,
                    onchange: move |e: Event<FormData>| {
                        let raw = e.value();
                        let value = raw.trim().parse().unwrap_or(1.5);
                        spread.set(value);
                    }
                }
            }
            div {
                class: "field",
                label {
                    for: "pip_places",
                    "Pip decimal places"
                }
                input {
                    id: "pip_places",
                    type: "number",
                    step: "1",
                    min: "0",
                    max: "10",
                    value: pip_places,
                    onchange: move |e: Event<FormData>| {
                        let raw = e.value();
                        let value = raw.trim().parse().unwrap_or(4);
                        pip_places.set(value);
                    }
                }
            }
            div {
                class: "field",
                label {
                    for: "high_low",
                    "Enter the M top level, or W low level"
                }
                input {
                    id: "high_low",
                    type: "number",
                    step: "any",
                    value: {
                        let decimal_places = *pip_places.read() + 1;
                        high_low.read().as_ref().map(|v| format!("{v:.decimal_places$}")).unwrap_or_default()
                    },
                    onchange: get_input(high_low)
                }
            }
            div {
                class: "field",
                label {
                    for: "neckline",
                    "Enter the neckline level"
                }
                input {
                    id: "neckline",
                    type: "number",
                    step: "any",
                    value: {
                        let decimal_places = *pip_places.read() + 1;
                        neckline.read().as_ref().map(|v| format!("{v:.decimal_places$}")).unwrap_or_default()
                    },
                    onchange: get_input(neckline)
                }
            }
            div {
                class: "results",
                Output {
                    high_low: high_low,
                    neckline: neckline,
                    pip_places: pip_places,
                    spread: spread,
                }
            }
        }
    }
}
