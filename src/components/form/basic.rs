use crate::*;

use super::inputs::*;

#[component]
pub fn Basic() -> Element {
    let State {
        mut main_parms,
        mut frame_parms,
        mut fading,
        // mut seed,
        ..
    } = use_context();

    rsx! {
        div { class: "w-full grid grid-cols-4 sm:grid-cols-6 xl:grid-cols-12 gap-1 sm:gap-y-1.5 sm:gap-x-3 md:gap-x-4",
            FloatInput {
                class: "col-start-1",
                name: "F1",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[0],
                min: 180.,
                max: frame_parms.read().oral_formant_freq[1].min(1300.),
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[0] = new_value;
                    }
                },
            }
            FloatInput {
                name: "F2",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[1],
                min: frame_parms.read().oral_formant_freq[0].max(550.),
                max: frame_parms.read().oral_formant_freq[2].min(3000.),
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[1] = new_value;
                    }
                },
            }
            FloatInput {
                name: "F3",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[2],
                min: frame_parms.read().oral_formant_freq[1].max(1200.),
                max: frame_parms.read().oral_formant_freq[3].min(4800.),
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[2] = new_value;
                    }
                },
            }
            FloatInput {
                name: "F4",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[3],
                min: frame_parms.read().oral_formant_freq[2].max(2000.),
                max: frame_parms.read().oral_formant_freq[4].min(5000.),
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[3] = new_value;
                    }
                },
            }
            FloatInput {
                name: "F5",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[4],
                min: frame_parms.read().oral_formant_freq[3].max(3000.),
                max: frame_parms.read().oral_formant_freq[5].min(5000.),
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[4] = new_value;
                    }
                },
            }
            FloatInput {
                name: "F6",
                unit: "Hz",
                value: frame_parms.read().oral_formant_freq[5],
                min: frame_parms.read().oral_formant_freq[4].max(3000.),
                max: 6000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_freq[5] = new_value;
                    }
                },
            }
            FloatInput {
                class: "col-start-1 xl:col-start-auto",
                name: "b1",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[0],
                min: 30.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[0] = new_value;
                    }
                },
            }
            FloatInput {
                name: "b2",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[1],
                min: 40.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[1] = new_value;
                    }
                },
            }
            FloatInput {
                name: "b3",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[2],
                min: 60.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[2] = new_value;
                    }
                },
            }
            FloatInput {
                name: "b4",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[3],
                min: 100.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[3] = new_value;
                    }
                },
            }
            FloatInput {
                name: "b5",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[4],
                min: 100.,
                max: 1500.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[4] = new_value;
                    }
                },
            }
            FloatInput {
                name: "b6",
                unit: "Hz",
                value: frame_parms.read().oral_formant_bw[5],
                min: 100.,
                max: 4000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().oral_formant_bw[5] = new_value;
                    }
                },
            }
            FloatInput {
                class: "col-start-1",
                name: "Fn",
                help: "Nasal formant frequency in Hz. Can be empty.",
                unit: "Hz",
                value: frame_parms.read().nasal_formant_freq,
                min: 100.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().nasal_formant_freq = new_value;
                    } else {
                        frame_parms.write().nasal_formant_freq = f64::NAN;
                    }
                },
                placeholder: "Ignored",
            }
            FloatInput {
                name: "bn",
                help: "Nasal formant bandwidth in Hz. Can be empty.",
                unit: "Hz",
                value: frame_parms.read().nasal_formant_bw,
                min: 40.,
                max: 1000.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().nasal_formant_bw = new_value;
                    } else {
                        frame_parms.write().nasal_formant_bw = f64::NAN;
                    }
                },
                placeholder: "Ignored",
            }
            FloatInput {
                name: "f0",
                help: "Fundamental frequency in Hz.",
                unit: "Hz",
                value: frame_parms.read().f0,
                min: 50.,
                max: 500.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().f0 = new_value;
                    }
                },
            }
            FloatInput {
                name: "Duration",
                unit: "s",
                value: frame_parms.read().duration,
                min: 0.1,
                max: 1.0,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().duration = new_value;
                    }
                },
            }
            FloatInput {
                name: "Fading",
                unit: "s",
                help: "Duration of fading in and out of the sound in seconds.",
                value: fading.read().clone(),
                min: 0.01,
                max: frame_parms.read().duration / 2.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        fading.set(new_value);
                    }
                },
            }
            FloatInput {
                name: "Flutter",
                help: "Flutter level.",
                value: frame_parms.read().flutter_level,
                min: 0.,
                max: 1.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().flutter_level = new_value;
                    }
                },
            }
            FloatInput {
                name: "Open Phase",
                help: "Relative length of the open phase of the glottis.",
                value: frame_parms.read().open_phase_ratio,
                min: 0.01,
                max: 1.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().open_phase_ratio = new_value;
                    }
                },
            }
            FloatInput {
                name: "Breath.",
                unit: "dB",
                help: "Breathiness in voicing (turbulence) in dB.",
                value: frame_parms.read().breathiness_db,
                min: -100.,
                max: 0.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().breathiness_db = new_value;
                    }
                },
            }
            FloatInput {
                name: "Tilt",
                unit: "dB",
                help: "Spectral tilt for glottal source in dB.\nAttenuation at 3 kHz in dB. 0 = no tilt.",
                value: frame_parms.read().tilt_db,
                min: -100.,
                max: 0.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().tilt_db = new_value;
                    }
                },
            }
            FloatInput {
                name: "Gain",
                unit: "dB",
                help: "Overall gain (output gain) in dB.\nLeave empty for automatic gain control (AGC).",
                value: frame_parms.read().gain_db,
                placeholder: "AGC ON",
                min: -100.,
                max: 0.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().gain_db = new_value;
                    } else {
                        frame_parms.write().gain_db = f64::NAN;
                    }
                },
            }
            FloatInput {
                name: "AGC",
                help: "RMS level for automatic gain control (AGC),\nonly relevant when Gain is empty.",
                value: frame_parms.read().agc_rms_level,
                placeholder: "Gain ON",
                min: 0.,
                max: 1.,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        frame_parms.write().agc_rms_level = new_value;
                    } else {
                        frame_parms.write().agc_rms_level = f64::NAN;
                    }
                },
            }
            EnumDropdown {
                name: "Glottal Source",
                help: "Type of glottal source.",
                value: main_parms.read().glottal_source_type,
                onchange: move |e: FormEvent| {
                    let value = e.value();
                    if let Ok(new_value) = value.parse() {
                        main_parms.write().glottal_source_type = new_value;
                    }
                },
            }
        }
    }
}
