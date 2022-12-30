use crate::audio_bus::AudioBus;
use crate::module::Module;
use crate::processor::Processor;
use rodio::{source::Source, OutputStream, OutputStreamHandle};

pub struct AudioOutput {
    pub input: AudioBus,
    pub output: AudioBus,
    stream_handle: (OutputStream, OutputStreamHandle),
}

impl Processor for AudioOutput {
    fn process(&mut self) {}
}

impl Module for AudioOutput {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            stream_handle: OutputStream::try_default().unwrap(),
        }
    }
}
