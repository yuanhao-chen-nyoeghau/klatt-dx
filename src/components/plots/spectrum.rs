use crate::*;

#[component]
pub fn Spectrum() -> Element {
    let State {
        fft_spectrum,
        envelope,
        ..
    } = use_context();

    let a_min = -120.;
    let a_max = 5.;
    let scale = 600;

    rsx! {
        Plotter {
            class: "border-base-content rounded border border-solid",
            size: (5 * scale, 1 * scale),
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
                chart
                    .draw_series(
                        DashedLineSeries::new(
                            envelope
                                .read()
                                .iter()
                                .take_while(|&(fr, _)| *fr <= FREQ_PLOT_MAX)
                                .map(|&(fr, amp)| (fr, amp)),
                            10,
                            5,
                            ShapeStyle::from(COLOR_INFO).stroke_width(2),
                        ),
                    )
                    .unwrap()
                    .label("Vocal Tract Transfer Function (Ideal Spectral Envelope)")
                    .legend(|(x, y)| {
                        Rectangle::new(
                            [(x - 20, y - 8), (x + 20, y - 4)],
                            ShapeStyle::from(COLOR_INFO).filled(),
                        )
                    });
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
                    .unwrap()
                    .label("FFT Spectrum")
                    .legend(|(x, y)| {
                        Rectangle::new(
                            [(x - 20, y - 8), (x + 20, y - 4)],
                            ShapeStyle::from(BLACK).filled(),
                        )
                    });
                chart
                    .configure_series_labels()
                    .label_font(("sans-serif", 30))
                    .position(SeriesLabelPosition::Coordinate(2170, 25))
                    .draw()
                    .unwrap();
            },
        }
    }
}
