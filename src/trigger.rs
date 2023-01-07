use crate::{bus::Bus, module::Module, processor::Processor};

pub struct Trigger {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    cycle: u32,
}

impl Processor for Trigger {
    fn process(&mut self) {
        self.cycle += 1;

        if self.cycle > crate::SAMPLE_RATE as u32 {
            self.audio_output.value = 1.0;
            self.cycle = 0;
        } else {
            self.audio_output.value = -1.0;
        }
    }
}

impl Module<f32> for Trigger {
    fn input(&mut self) -> &Bus<f32> {
        &self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &self.audio_output
    }
}

impl Trigger {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            cycle: 0,
        }
    }
}
