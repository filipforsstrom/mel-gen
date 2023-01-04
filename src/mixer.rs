use crate::{bus::Bus, module::Module, processor::Processor};

pub struct Mixer {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
}

impl Processor for Mixer {
    fn process(&mut self) {
        self.audio_output.value = self.audio_input.value;
    }
}

impl Module<f32> for Mixer {
    fn input(&mut self) -> &Bus<f32> {
        &self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &self.audio_output
    }
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
        }
    }
}
