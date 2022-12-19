use nih_plug::prelude::*;
use crate::voice::*;
use std::f32::consts;

#[derive(Enum, PartialEq)]
pub enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
}

#[derive(Params)]
pub struct OscillatorParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "waveform"]
    pub waveform: EnumParam<Waveform>,

    #[id = "enabled"]
    pub enabled: BoolParam,
}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-6.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 0.0)
                }
            )
            .with_unit("dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            waveform: EnumParam::new("Waveform", Waveform::Sine),
            enabled: BoolParam::new("Enabled", false),
        }
    }
}

pub fn calculate_amplitude(oscillator_params: &[OscillatorParams], voice: &mut Voice, sample_rate: f32) -> f32 {
    let phase = voice.phase;
    voice.phase += voice.get_frequency() / sample_rate;

    if voice.phase >= 1.0 {
        voice.phase -= 1.0;
    }
    
    let mut amp = 0.0;
    oscillator_params.iter().for_each(|oscillator| {
        if oscillator.enabled.value() == false {
            return;
        }

        amp += match oscillator.waveform.value() {
            Waveform::Sine => (phase * consts::TAU).sin(),
            Waveform::Saw => phase * -2.0 + 1.0,
            Waveform::Square => if phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Triangle => ((phase * consts::TAU).sin() * 2.0 / consts::PI).asin(), 
        } * oscillator.gain.value();
    });

    amp * voice.velocity
}
