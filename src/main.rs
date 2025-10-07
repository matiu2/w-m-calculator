use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

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
            {
                if let (Some(high_low_val), Some(neckline_val)) = (*high_low.read(), *neckline.read()) {
                    let pip_size = 10.0_f64.powi(-(*pip_places.read() as i32));
                    let is_w = neckline_val > high_low_val;
                    let stop_loss = high_low_val;
                    let (entry, sl) = if is_w {
                        (
                            neckline_val + (0.5 + *spread.read()) * pip_size,
                            stop_loss - 1.0 * pip_size,
                        )
                    } else {
                        (
                            neckline_val - 0.5 * pip_size,
                            stop_loss + (1.0 + *spread.read()) * pip_size,
                        )
                    };
                    let tp = if is_w {
                        entry + (entry - sl)
                    } else {
                        entry - (sl - entry)
                    };
                    let decimal_places = *pip_places.read() + 1;
                    let entry_str = format!("{entry:.decimal_places$}");
                    let sl_str = format!("{sl:.decimal_places$}");
                    let tp_str = format!("{tp:.decimal_places$}");
                    let pattern_title = if is_w {
                        "This is a W pattern, longing"
                    } else {
                        "This is an M pattern, shorting"
                    };
                    rsx! {
                        h3 {
                            class: "pattern-title",
                            "{pattern_title}"
                        }
                        dl {
                            dt { "Entry:" }
                            dd {
                                class: "output-value",
                                span { "{entry_str}" }
                                button {
                                    class: "copy-btn",
                                    onclick: move |_| {
                                        eval(&format!(r#"navigator.clipboard.writeText("{entry_str}")"#));
                                    },
                                    "📋"
                                }
                            }
                            dt { "Stop loss:" }
                            dd {
                                class: "output-value",
                                span { "{sl_str}" }
                                button {
                                    class: "copy-btn",
                                    onclick: move |_| {
                                        eval(&format!(r#"navigator.clipboard.writeText("{sl_str}")"#));
                                    },
                                    "📋"
                                }
                            }
                            dt { "Take Profit:" }
                            dd {
                                class: "output-value",
                                span { "{tp_str}" }
                                button {
                                    class: "copy-btn",
                                    onclick: move |_| {
                                        eval(&format!(r#"navigator.clipboard.writeText("{tp_str}")"#));
                                    },
                                    "📋"
                                }
                            }
                        }
                    }
                } else {
                    rsx! {p{"Need both values to calc"}}
                }
            }
        }
    }
}
