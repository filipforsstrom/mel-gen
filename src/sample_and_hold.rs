use crate::{bus::Bus, module::AudioModule, processor::Processor};

pub struct SampleAndHold {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
    pub trigger: Bus<f32>,
}

impl Processor for SampleAndHold {
    fn process(&mut self) {
        if self.trigger.value > 0.0 {
            self.output.value = self.input.value;
        }
    }
}

impl AudioModule for SampleAndHold {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.output
    }
}

impl SampleAndHold {
    pub fn new() -> Self {
        Self {
            input: Bus::<f32>::new(),
            output: Bus::<f32>::new(),
            trigger: Bus::<f32>::new(),
        }
    }
}
