use std::{error, slice};

use num::{
    complex::{Complex, Complex64},
    traits::FloatConst,
    Float,
};
use rand::{rngs::SmallRng, SeedableRng};
use spectrum_analyzer::{
    samples_fft_to_spectrum, scaling::divide_by_N_sqrt, windows::hann_window, FrequencyLimit,
};

use crate::*;

pub fn update_results() -> Result<(), Box<dyn error::Error + 'static>> {
    let State {
        main_parms,
        frame_parms,
        fading,
        mut envelope,
        mut sound,
        mut fft_spectrum,
        seed,
        ..
    } = use_context();

    // envelope
    let trans =
        get_vocal_tract_transfer_function_coefficients(&main_parms.read(), &frame_parms.read())?;
    let pi = f64::PI();
    let f_scaling = 2. * pi / main_parms.read().sample_rate as f64;
    let abs_vocal_tract_spectrum_fn = |f: f64| {
        let w = f * f_scaling;
        if w < 0. || w >= pi {
            return f64::NAN;
        }
        let z = Complex64::cis(w);
        let r = poly_fraction_complex64_eval(&trans, z);
        let a = r.norm();
        let db = amp_to_db(a);
        db
    };
    let mut oral_formant_freqs = vec![0.];
    oral_formant_freqs.extend_from_slice(&frame_parms.read().oral_formant_freq);
    let max_db = oral_formant_freqs
        .iter()
        .map(|&f| abs_vocal_tract_spectrum_fn(f))
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    envelope.set(
        (0..(FREQ_PLOT_MAX) as usize)
            .map(|i| i as f64)
            .map(|f| (f, abs_vocal_tract_spectrum_fn(f) - max_db))
            .collect(),
    );

    // sound
    let rng = SmallRng::seed_from_u64(*seed.read());
    let mut generated_sound = generate_sound(
        &main_parms.read(),
        slice::from_ref(&frame_parms.read()),
        rng,
    )?;

    // fft
    let generated_sound_len = generated_sound.len();
    let prev_power_of_2 = (generated_sound_len / 2).next_power_of_two();
    let selected_len = prev_power_of_2.min(2usize.pow(15));
    let start_idx = (generated_sound_len - selected_len) / 2;
    let hann_window = hann_window(
        &generated_sound[start_idx..start_idx + selected_len]
            .iter()
            .map(|x| *x as f32)
            .collect::<Vec<_>>(),
    );
    if let Ok(spectrum_hann_window) = samples_fft_to_spectrum(
        // (windowed) samples
        &hann_window,
        // sampling rate
        main_parms.read().sample_rate as u32,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::Max(main_parms.read().sample_rate as f32 / 2.),
        // optional scale
        Some(&divide_by_N_sqrt),
    ) {
        fft_spectrum.set(
            spectrum_hann_window
                .data()
                .iter()
                .map(|(fr, fr_val)| (fr.val() as f64, amp_to_db(fr_val.val()) as f64 - max_db))
                .collect(),
        );
    } else {
        return Err("Failed to compute FFT spectrum".into());
    }

    // fading
    let fading = *fading.read();
    if fading > 0.0 {
        let n_samples = generated_sound.len();
        let duration = frame_parms.read().duration;
        let fade_in_end = (fading / duration) * n_samples as f64;
        let fade_out_start = n_samples as f64 - (fading / duration) * n_samples as f64;
        for i in 0..n_samples {
            let x = i as f64;
            if x < fade_in_end {
                generated_sound[i] *= x / fade_in_end;
            } else if x > fade_out_start {
                let fade_out_factor = (n_samples as f64 - x) / (n_samples as f64 - fade_out_start);
                generated_sound[i] *= fade_out_factor;
            }
        }
        sound.set(generated_sound);
    }

    Ok(())
}

fn amp_to_db<F: num::Float + From<i16>>(amp: F) -> F {
    if amp.le(&F::zero()) || amp.is_nan() {
        F::neg_infinity()
    } else {
        <F as From<_>>::from(20) * amp.log10()
    }
}

fn poly_complex_eval<F: Float + FloatConst>(poly: &[F], z: Complex<F>) -> Complex<F> {
    let n = poly.len() - 1;
    let mut r = Complex::new(poly[n], F::zero());
    for i in (0..n).rev() {
        r = r * z + Complex::new(poly[i], F::zero());
    }
    r
}

fn poly_fraction_complex64_eval(fraction: &Fraction, z: Complex64) -> Complex64 {
    let Fraction(numerator, denominator) = fraction;
    poly_complex_eval(&numerator, z).fdiv(poly_complex_eval(&denominator, z))
}

pub fn play_sound(sound: &[f64], sample_rate: f64) -> Result<(), JsValueError> {
    let State {
        mut audio_playing,
        audio_context,
        ..
    } = use_context();
    let audio_buffer =
        audio_context
            .read()
            .create_buffer(1, sound.len() as u32, sample_rate as f32)?;

    audio_buffer.copy_to_channel(&sound.iter().map(|&x| x as f32).collect::<Vec<_>>(), 0)?;

    let audio_buffer_source = audio_context.read().create_buffer_source()?;
    audio_buffer_source.set_buffer(Some(&audio_buffer));
    audio_buffer_source.set_loop(false);
    audio_buffer_source.connect_with_audio_node(&audio_context.read().destination())?;

    let onended = Closure::<dyn FnMut(JsValue)>::new(move |_| {
        audio_playing.set(false);
    });
    audio_buffer_source
        .add_event_listener_with_callback("ended", onended.as_ref().unchecked_ref())?;
    onended.forget();

    audio_buffer_source.start()?;
    Ok(())
}
