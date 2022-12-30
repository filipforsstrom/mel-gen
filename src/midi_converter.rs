use crate::{audio_bus::AudioBus, module::Module, processor::Processor};

pub struct MidiConverter {
    pub input: AudioBus,
    pub output: AudioBus,
    pub midi_output: u8,
}

impl Processor for MidiConverter {
    fn process(&mut self) {
        self.output.value = self.input.value;
        self.midi_output = ((self.input.value + 1.0) * 127.0 / 2.0) as u8;
    }
}

impl Module for MidiConverter {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl MidiConverter {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            midi_output: 0,
        }
    }
}
