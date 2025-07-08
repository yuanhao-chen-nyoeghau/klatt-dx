use klsyn_dioxus::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(State::default);

    use_effect(move || {
        update_results().unwrap();
    });

    rsx! {
        document::Link { rel: "stylesheet", href: "https://rsms.me/inter/inter.css" }
        document::Link { rel: "stylesheet", href: asset!("/tailwind.css") }

        body { class: "container mx-auto px-5 py-10",
            div { class: "max-w-[1400px] mx-auto",
                h1 { "Klsyn Formant Synthesizer" }
                Plots {}
                Buttons {}
                Form {}
            }
        }
    }
}
