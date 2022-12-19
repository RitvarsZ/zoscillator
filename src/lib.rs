mod oscillator;
mod voice;

use nih_plug::prelude::*;
use std::sync::Arc;
use oscillator::*;
use voice::*;

struct Zoscillator {
    params: Arc<ZoscillatorParams>,
    voices: Vec<Voice>,
    sample_rate: f32,
}

#[derive(Params)]
struct ZoscillatorParams {
    #[nested(array, group = "Oscillators")]
    pub oscillators: [OscillatorParams; 3],
}

impl Default for Zoscillator {
    fn default() -> Self {
        let params = Arc::new(ZoscillatorParams::default());
        Self {
            params,
            voices: Vec::new(),
            sample_rate: 1.0,
        }
    }
}

impl Zoscillator {
    fn calculate_amplitude(&mut self) -> f32 {
        self.voices.iter_mut().fold(0.0, |acc, voice| {
            acc + calculate_amplitude(
                &self.params.oscillators,
                voice,
                self.sample_rate
            )
        })
    }

    fn handle_note_event(&mut self, event: NoteEvent) {
        match event {
            NoteEvent::NoteOn { note, velocity, .. } => {
                self.voices.push(Voice::new(note, velocity.sqrt()));
                nih_dbg!("Note on: {}", note);
            },
            NoteEvent::NoteOff { note, .. } => {
                if let Some(index) = self.voices.iter()
                    .position(|voice| voice.note == note) {
                        self.voices.remove(index);
                        nih_dbg!("Note off: {}", note);
                    };
            },
            _ => {}
        };
    }
}

impl Default for ZoscillatorParams {
    fn default() -> Self {
        let oscillators = [1, 2, 3].map(|index| {
            let mut osc = OscillatorParams::default();

            if index == 1 {
                osc.enabled = BoolParam::new("Enabled", true);
            }

            osc
        });

        Self {
            oscillators,
        }
    }
}

impl Plugin for Zoscillator {
    const NAME: &'static str = "Zoscillator";
    const VENDOR: &'static str = "RitvarsZ";
    const URL: &'static str = "https://github.com/RitvarsZ";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        config.num_input_channels == 0 && config.num_output_channels > 0
    }

    fn initialize(
            &mut self,
            _bus_config: &BusConfig,
            buffer_config: &BufferConfig,
            _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        true
    }

    fn reset(&mut self) {
        self.voices.clear();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                // if the event is in the future, break out of the loop
                if event.timing() > sample_id as u32 {
                    break;
                }

                self.handle_note_event(event);

                next_event = context.next_event();
            }

            let amplitude: f32 = self.calculate_amplitude();

            for channel_sample in channel_samples {
                *channel_sample = amplitude;
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl Vst3Plugin for Zoscillator {
    const VST3_CLASS_ID: [u8; 16] = *b"RitvarsZ\0\0\0\0\0\0\0\0";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth";
}

nih_export_vst3!(Zoscillator);