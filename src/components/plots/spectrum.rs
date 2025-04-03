use crate::*;

#[component]
pub fn Spectrum() -> Element {
    let State {
        fft_spectrum,
        envelope,
        frame_parms,
        ..
    } = use_context();

    let a_min = -120.;
    let a_max = 5.;

    rsx! {
        Plotters {
            class: "border-base-content rounded border border-solid",
            size: (2500, 500),
            draw: move |drawing_area| {
                let mut chart = ChartBuilder::on(&drawing_area)
                    .margin(25)
                    .x_label_area_size(10)
                    .y_label_area_size(35)
                    .build_cartesian_2d(0f64..FREQ_PLOT_MAX, a_min..a_max)
                    .unwrap();
                chart
                    .configure_mesh()
                    .label_style(("sans-serif", 30))
                    .x_label_formatter(&|x| format!("{:.0}", x))
                    .y_label_formatter(&|y| format!("{:.0}", y))
                    .max_light_lines(0)
                    .draw()
                    .unwrap();
                let f0 = frame_parms.read().f0;
                chart
                    .draw_series(
                        LineSeries::new(
                            vec![(f0, a_min), (f0, a_max)],
                            ShapeStyle::from(COLOR_NEUTRAL).stroke_width(2),
                        ),
                    )
                    .unwrap();
                chart
                    .draw_series(
                        LineSeries::new(
                            envelope
                                .read()
                                .iter()
                                .take_while(|&(fr, _)| *fr <= FREQ_PLOT_MAX)
                                .map(|&(fr, amp)| (fr, amp)),
                            ShapeStyle::from(BLACK).stroke_width(2),
                        ),
                    )
                    .unwrap();
                chart
                    .draw_series(
                        LineSeries::new(
                            fft_spectrum
                                .read()
                                .iter()
                                .take_while(|&(fr, _)| *fr <= FREQ_PLOT_MAX)
                                .map(|&(fr, amp)| (fr, amp)),
                            BLACK,
                        ),
                    )
                    .unwrap();
            },
        }
    }
}
