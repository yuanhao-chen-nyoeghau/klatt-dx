use crate::*;

#[component]
pub fn Waveform() -> Element {
    let State { sound, .. } = use_context();

    rsx! {
        Plotter {
            class: "border-base-content rounded border border-solid",
            size: (2500, 300),
            draw: move |drawing_area| {
                let mut chart = ChartBuilder::on(&drawing_area)
                    .build_cartesian_2d(0..sound.len(), -0.75f64..0.75)
                    .unwrap();
                chart
                    .draw_series(
                        LineSeries::new((0..sound.len()).map(|i| (i, sound.read()[i])), BLACK),
                    )
                    .unwrap();
            },
        }
    }
}
