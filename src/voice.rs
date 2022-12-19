use nih_plug::prelude::util;

pub struct Voice {
  pub phase: f32,
  pub velocity: f32,
  pub note: u8,
}

impl Voice {
  pub fn new(note: u8, velocity: f32) -> Self {
    Self {
      phase: 0.0,
      velocity,
      note,
    }
  }

  pub fn get_frequency(&self) -> f32 {
    util::midi_note_to_freq(self.note)
  }
}
