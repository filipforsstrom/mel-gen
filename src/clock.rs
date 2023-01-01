use crate::{bus::Bus, module::AudioModule, processor::Processor};

pub struct Clock {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
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

impl AudioModule for Clock {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.output
    }
}

impl Clock {
    pub fn new() -> Self {
        Self {
            input: Bus::<f32>::new(),
            output: Bus::<f32>::new(),
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
