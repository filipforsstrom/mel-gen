use crate::{
    bus::Bus,
    module::{AudioModule, MidiModule},
    processor::Processor,
};

pub struct Quantizer {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
}

impl Processor for Quantizer {
    fn process(&mut self) {
        let scale = [0, 2, 3, 5, 7, 8, 10];
        let octave = self.midi_input.value / 12;
        let within_octave = self.midi_input.value % 12;

        let mut nearest_distance = std::u8::MAX;
        let mut nearest_note = 0;
        for (i, &note) in scale.iter().enumerate() {
            let distance = if note > within_octave {
                note - within_octave
            } else {
                within_octave - note
            };
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_note = i;
            }
        }
        self.midi_output.value = octave * 12 + scale[nearest_note]
    }
}

impl AudioModule for Quantizer {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.audio_input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.audio_output
    }
}

impl MidiModule for Quantizer {
    fn midi_input(&mut self) -> &Bus<u8> {
        &mut self.midi_input
    }

    fn midi_output(&mut self) -> &Bus<u8> {
        &mut self.midi_output
    }
}

impl Quantizer {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
        }
    }
}
