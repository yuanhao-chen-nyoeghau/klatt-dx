use crate::*;

#[component]
pub fn Buttons() -> Element {
    let State {
        main_parms, sound, ..
    } = use_context();

    let play_sound = move |_| {
        play_sound(&sound.read(), main_parms.read().sample_rate as f64).unwrap();
    };

    rsx! {
        div { class: "my-3",
            button { class: "btn btn-outline btn-sm", onclick: play_sound, "Play" }
        }
    }
}
