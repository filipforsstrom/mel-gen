use crate::bus::Bus;
use crate::module::Module;
use crate::processor::Processor;

pub struct AudioOutput {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
}

impl Processor for AudioOutput {
    fn process(&mut self) {}
}

impl Module<f32> for AudioOutput {
    fn input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn output(&mut self) -> &Bus<f32> {
        &mut self.output
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            input: Bus::<f32>::new(),
            output: Bus::<f32>::new(),
        }
    }
}
