use web_sys::AudioContext;

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub main_parms: Signal<MainParms>,
    pub frame_parms: Signal<FrameParms>,

    pub fading: Signal<f64>,
    pub seed: Signal<u64>,

    pub sound: Signal<Vec<f64>>,
    pub fft_spectrum: Signal<Vec<(f64, f64)>>,
    pub envelope: Signal<Vec<(f64, f64)>>,

    pub audio_context: Signal<AudioContext>,
}

impl Default for State {
    fn default() -> Self {
        State {
            main_parms: Signal::new(DEFAULT_MAIN_PARMS),
            frame_parms: Signal::new(DEFAULT_FRAME_PARMS),
            fading: Signal::new(0.03),
            seed: Signal::new({
                let now = web_time::SystemTime::now();
                now.duration_since(web_time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0) as u64
            }),
            sound: Signal::new(Vec::new()),
            fft_spectrum: Signal::new(Vec::new()),
            envelope: Signal::new(Vec::new()),
            audio_context: Signal::new(AudioContext::new().unwrap()),
        }
    }
}
