pub use plotters::prelude::*;

use crate::*;

mod spectrum;
mod waveform;

use spectrum::*;
use waveform::*;

#[component]
pub fn Plots() -> Element {
    rsx! {
        div { class: "my-3", Waveform {} }
        div { class: "my-3", Spectrum {} }
    }
}
