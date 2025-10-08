use dioxus::{document::eval, prelude::*};

#[component]
pub fn Output(
    high_low: Signal<Option<f64>>,
    neckline: Signal<Option<f64>>,
    pip_places: Signal<usize>,
    spread: Signal<f64>,
) -> Element {
    if let (Some(high_low_val), Some(neckline_val)) = (*high_low.read(), *neckline.read()) {
        let high_low = use_signal(|| high_low_val);
        let neckline = use_signal(|| neckline_val);
        rsx! {
            OutputWithValues {
                high_low: high_low,
                neckline: neckline,
                pip_places: pip_places,
                spread: spread,
            }
        }
    } else {
        rsx! {p{"Need all values to calc"}}
    }
}

#[component]
pub fn OutputWithValues(
    high_low: Signal<f64>,
    neckline: Signal<f64>,
    pip_places: Signal<usize>,
    spread: Signal<f64>,
) -> Element {
    let neckline_val = *neckline.read();
    let high_low_val = *high_low.read();
    let pip_size = 10.0_f64.powi(-(*pip_places.read() as i32));
    let is_w = neckline_val > high_low_val;
    let stop_loss = high_low_val;
    let (entry, sl) = if is_w {
        // For W patterns (long position)
        // Our inputs are bid prices, and we will be buying at the
        // ask price, so we need to add the broker spread
        (
            neckline_val + (0.5 + *spread.read()) * pip_size,
            stop_loss - 1.0 * pip_size,
        )
    } else {
        // For M patterns (short positions)
        // Our stoploss input is the bid price, but if it
        // gets hit, it'll be the ask price that hits it
        // so we need to add the borker spread
        (
            neckline_val - 0.5 * pip_size,
            stop_loss + (1.0 + *spread.read()) * pip_size,
        )
    };
    // Take profit is 1:1
    let tp = if is_w {
        entry + (entry - sl)
    } else {
        entry - (sl - entry)
    };

    // Calculate distances in pips
    let distance_to_sl_pips = (entry - sl).abs() / pip_size;
    let distance_to_tp_pips = (tp - entry).abs() / pip_size;
    let spread_ratio = distance_to_sl_pips / *spread.read();

    let decimal_places = *pip_places.read() + 1;
    let entry_str = format!("{entry:.decimal_places$}");
    let sl_str = format!("{sl:.decimal_places$}");
    let tp_str = format!("{tp:.decimal_places$}");
    let distance_to_sl_str = format!("{distance_to_sl_pips:.1}");
    let distance_to_tp_str = format!("{distance_to_tp_pips:.1}");
    let spread_ratio_str = format!("{spread_ratio:.1}");
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
        if spread_ratio < 10.0 {
            div {
                class: "alert-box",
                "⚠️ WARNING: Spread ratio is {spread_ratio_str}x - this is less than 10x the broker spread!"
            }
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
            dt { "Distance to SL (pips):" }
            dd {
                class: "output-value info-value",
                span { "{distance_to_sl_str}" }
            }
            dt { "Distance to TP (pips):" }
            dd {
                class: "output-value info-value",
                span { "{distance_to_tp_str}" }
            }
            dt { "Spread Ratio (distance/spread):" }
            dd {
                class: "output-value info-value",
                span { "{spread_ratio_str}x" }
            }
        }
    }
}
