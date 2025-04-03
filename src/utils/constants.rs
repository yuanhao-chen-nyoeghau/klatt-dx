use crate::*;

pub const FREQ_PLOT_MAX: f64 = 6000.;

pub const DEFAULT_MAIN_PARMS: MainParms = MainParms {
    sample_rate: 44100,
    glottal_source_type: GlottalSourceType::Natural,
};
pub const DEFAULT_FRAME_PARMS: FrameParms = FrameParms {
    duration: 0.75,
    f0: 174.61,
    flutter_level: 0.25,
    open_phase_ratio: 0.7,
    breathiness_db: -25.,
    tilt_db: 0.,
    gain_db: f64::NAN,
    agc_rms_level: 0.25,
    nasal_formant_freq: f64::NAN,
    nasal_formant_bw: f64::NAN,
    oral_formant_freq: [520., 1006., 2831., 3168., 4135., 5020.],
    oral_formant_bw: [76., 102., 72., 102., 816., 596.],
    cascade_enabled: true,
    cascade_voicing_db: 0.,
    cascade_aspiration_db: -25.,
    cascade_aspiration_mod: 0.5,
    nasal_antiformant_freq: f64::NAN,
    nasal_antiformant_bw: f64::NAN,
    parallel_enabled: false,
    parallel_voicing_db: 0.,
    parallel_aspiration_db: -25.,
    parallel_aspiration_mod: 0.5,
    frication_db: -30.,
    frication_mod: 0.5,
    parallel_bypass_db: -99.,
    nasal_formant_db: f64::NAN,
    oral_formant_db: [0., -8., -15., -19., -30., -35.],
};
