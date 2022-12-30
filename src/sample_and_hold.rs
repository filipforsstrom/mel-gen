use crate::{audio_bus::AudioBus, module::Module, processor::Processor};

pub struct SampleAndHold {
    pub input: AudioBus,
    pub output: AudioBus,
    pub trigger: AudioBus,
}

impl Processor for SampleAndHold {
    fn process(&mut self) {
        if self.trigger.value > 0.0 {
            self.output.value = self.input.value;
        }
    }
}

impl Module for SampleAndHold {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl SampleAndHold {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            trigger: AudioBus::new(),
        }
    }
}
