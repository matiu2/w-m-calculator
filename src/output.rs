use dioxus::{document::eval, prelude::*};

#[component]
pub fn Output(
    high_low: Signal<Option<f64>>,
    neckline: Signal<Option<f64>>,
    pip_places: Signal<usize>,
    spread: Signal<f64>,
) -> Element {
    if let (Some(high_low_val), Some(neckline_val)) = (*high_low.read(), *neckline.read()) {
        rsx! {
            OutputWithValues {
                high_low: high_low_val,
                neckline: neckline_val,
                pip_places: pip_places,
                spread: spread,
            }
        }
    } else {
        rsx! {
            div {
                class: "empty-state",
                "Need all values to calc"
            }
        }
    }
}

#[component]
pub fn OutputWithValues(
    high_low: f64,
    neckline: f64,
    pip_places: Signal<usize>,
    spread: Signal<f64>,
) -> Element {
    let mut copy_alert = use_signal(|| Option::<String>::None);

    let neckline_val = neckline;
    let high_low_val = high_low;
    let pip_size = 10.0_f64.powi(-(*pip_places.read() as i32));
    let is_w = neckline_val > high_low_val;
    let stop_loss = high_low_val;
    let spread_pips = *spread.read() * pip_size;
    let half_pip = 0.5 * pip_size;
    let one_pip = pip_size;

    // Price level adjustments (bid inputs, ask executions):
    //  | Level       | W Bottom (Long)                  | M Top (Short)                    |
    //  |-------------|----------------------------------|----------------------------------|
    //  | Entry       | Neckline + 0.5 pip               | Neckline - 0.5 pip               |
    //  | Stop Loss   | Wick low - (spread + 1.0)        | Wick high + (spread + 1.0)       |
    //  | Take Profit | Neckline + 1R - (spread + 0.5)   | Neckline - 1R + (spread + 0.5)   |
    let entry = if is_w {
        // Long entries buy through the ask; display the bid level with the buffer only
        neckline_val + half_pip
    } else {
        // Short entries sell at the bid, buffering just below the neckline
        neckline_val - half_pip
    };

    let sl = if is_w {
        // Long stop loss sits below the wick by the spread plus an extra pip safety
        stop_loss - (spread_pips + one_pip)
    } else {
        // Short stop loss sits above the wick by the spread plus one pip
        stop_loss + (spread_pips + one_pip)
    };

    let risk = if is_w {
        neckline_val - sl
    } else {
        sl - neckline_val
    };

    let tp = if is_w {
        // Move 1R above the neckline and remove the buffer on the way out
        neckline_val + risk - (spread_pips + half_pip)
    } else {
        // For shorts, project 1R below the neckline and add the buffer back
        neckline_val - risk + (spread_pips + half_pip)
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

    // Helper to copy and show alert
    let mut copy_value = move |value: &str, label: &str| {
        eval(&format!(r#"navigator.clipboard.writeText("{value}")"#));
        let msg = format!("{label} copied!");
        copy_alert.set(Some(msg));
    };

    // Clear alert after 2 seconds
    use_effect(move || {
        if copy_alert.read().is_some() {
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                copy_alert.set(None);
            });
        }
    });

    // Clone strings for keyboard handler
    let entry_str_kb = entry_str.clone();
    let sl_str_kb = sl_str.clone();
    let tp_str_kb = tp_str.clone();

    rsx! {
        // Keyboard event listener
        div {
            class: "result-panel",
            onkeydown: move |evt| {
                match evt.code() {
                    Code::KeyE => copy_value(&entry_str_kb, "Entry price"),
                    Code::KeyS => copy_value(&sl_str_kb, "Stop loss"),
                    Code::KeyT => copy_value(&tp_str_kb, "Take profit"),
                    _ => {}
                }
            },
            tabindex: 0,
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
            if let Some(alert_msg) = copy_alert.read().as_ref() {
                div {
                    class: "copy-alert",
                    "{alert_msg}"
                }
            }
            dl {
                dt { "Entry (e):" }
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
                dt { "Stop loss (s):" }
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
                dt { "Take Profit (t):" }
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
}
