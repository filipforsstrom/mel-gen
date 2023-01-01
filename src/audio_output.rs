use crate::bus::Bus;
use crate::module::AudioModule;
use crate::processor::Processor;
use rodio::{source::Source, OutputStream, OutputStreamHandle};

pub struct AudioOutput {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
    stream_handle: (OutputStream, OutputStreamHandle),
}

impl Processor for AudioOutput {
    fn process(&mut self) {}
}

impl AudioModule for AudioOutput {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.output
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            input: Bus::<f32>::new(),
            output: Bus::<f32>::new(),
            stream_handle: OutputStream::try_default().unwrap(),
        }
    }
}
