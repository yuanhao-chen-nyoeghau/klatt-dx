use std::fmt;

use strum::IntoEnumIterator;

use crate::*;

#[component]
fn LabelHelp(name: String, unit: Option<String>, help: Option<String>) -> Element {
    let underline_class = if let Some(_) = help {
        "underline decoration-dotted"
    } else {
        ""
    };

    let unit = rsx! {
        div { class: "text-base-content/70",
            if let Some(unit) = unit {
                "({unit})"
            } else {
                ""
            }
        }
    };
    let label_text = rsx! {
        div { class: "{underline_class} font-semibold", "{name}" }
    };
    let label_help = match help {
        Some(help_text) => {
            rsx! {
                span { class: "tooltip",
                    span { class: "tooltip-content whitespace-pre-line text-xs", {help_text} }
                    {label_text}
                }
            }
        }
        None => label_text,
    };

    rsx! {
        div { class: "flex flex-wrap justify-between text-xs",
            {label_help}
            {unit}
        }
    }
}

#[component]
pub fn FloatInput<T: fmt::Display + num::Float + 'static>(
    name: String,
    unit: Option<String>,
    help: Option<String>,
    class: Option<String>,
    value: T,
    min: T,
    max: T,
    placeholder: Option<String>,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: if let Some(c) = class { c },
            label { class: "flex flex-col justify-between h-full",
                LabelHelp { name, unit, help }
                div { class: "flex flex-col flex-initial",
                    input {
                        r#type: "number",
                        class: "input input-xs w-full h-4 validator text-right placeholder:text-center",
                        placeholder: placeholder.unwrap_or_default(),
                        step: 0.01,
                        min: "{min}",
                        max: "{max}",
                        value: "{value:.2}",
                        onchange: move |e| {
                            onchange.call(e);
                        },
                    }
                    input {
                        r#type: "range",
                        class: "range w-full h-2 [--range-fill:0] [--range-thumb-size:calc(var(--size-selector,.25rem))] mt-0.5",
                        step: 0.01,
                        min: "{min}",
                        max: "{max}",
                        value: "{value}",
                        oninput: move |e| {
                            onchange.call(e);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn EnumDropdown<T: IntoEnumIterator + Clone + PartialEq + fmt::Display + 'static>(
    name: String,
    help: Option<String>,
    class: Option<String>,
    value: T,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: if let Some(c) = class { c },
            label { class: "flex flex-col justify-between h-full",
                LabelHelp { name, unit: None, help }
                select {
                    class: "select select-xs h-full w-full",
                    value: "{value}",
                    oninput: move |e| {
                        onchange.call(e);
                    },
                    for variant in T::iter() {
                        option {
                            value: variant.to_string(),
                            selected: value == variant,
                            "{variant}"
                        }
                    }
                }
            }
        }
    }
}
