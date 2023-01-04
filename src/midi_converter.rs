use crate::{bus::Bus, module::Module, processor::Processor};

pub struct MidiConverter {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
}

impl Processor for MidiConverter {
    fn process(&mut self) {
        self.midi_output.value = ((self.audio_input.value + 1.0) * 127.0 / 2.0) as u8;
    }
}

impl Module<f32> for MidiConverter {
    fn input(&mut self) -> &Bus<f32> {
        &mut self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &mut self.audio_output
    }
}

impl Module<u8> for MidiConverter {
    fn input(&mut self) -> &Bus<u8> {
        &mut self.midi_input
    }

    fn output(&mut self) -> &Bus<u8> {
        &mut self.midi_output
    }
}

impl MidiConverter {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
        }
    }
}
