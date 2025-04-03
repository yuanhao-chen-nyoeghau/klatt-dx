use crate::*;

mod basic;
pub(in crate::components::form) mod inputs;

use basic::*;

#[component]
pub fn Form() -> Element {
    rsx! {
        div { class: "my-3 tabular-nums",
            h2 { class: "text-sm font-light text-base-content mb-1", "Parameters" }
            Basic {}
        }
    }
}
