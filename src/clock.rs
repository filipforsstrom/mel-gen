use crate::{audio_bus::AudioBus, module::Module, processor::Processor};

pub struct Clock {
    pub input: AudioBus,
    pub output: AudioBus,
    rate: f32,
    samples_per_cycle: u64,
    sample_counter: u64,
    trigger_length: u64,
}

impl Processor for Clock {
    fn process(&mut self) {
        // Increment the sample counter
        self.sample_counter += 1;

        if self.sample_counter >= self.samples_per_cycle {
            self.sample_counter = 0;
        }

        if self.sample_counter < self.trigger_length {
            self.output.value = 1.0;
        } else {
            self.output.value = -1.0;
        }
    }
}

impl Module for Clock {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl Clock {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            rate: 10.0,
            samples_per_cycle: (crate::SAMPLE_RATE / 10.0) as u64,
            sample_counter: 0,
            trigger_length: 1,
        }
    }
    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
        self.samples_per_cycle = (crate::SAMPLE_RATE / rate) as u64;
    }
}
