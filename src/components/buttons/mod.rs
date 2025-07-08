use crate::*;

#[component]
pub fn Buttons() -> Element {
    let State {
        main_parms,
        sound,
        mut audio_playing,
        mut audio_context,
        ..
    } = use_context();

    rsx! {
        div { class: "my-3",
            button {
                class: "btn btn-outline btn-sm w-24",
                onclick: move |_| {
                    if *audio_playing.read() {
                        let _ = audio_context.read().close();
                        audio_context.set(AudioContext::new().unwrap());
                        audio_playing.set(false);
                    } else {
                        play_sound(&sound.read(), main_parms.read().sample_rate as f64).unwrap();
                        audio_playing.set(true);
                    }
                },
                if *audio_playing.read() {
                    "Pause"
                } else {
                    "Play"
                }
            }
        }
    }
}
