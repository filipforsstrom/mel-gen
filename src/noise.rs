use crate::audio_bus::AudioBus;
use crate::module::Module;
use crate::processor::Processor;
use rand::{thread_rng, Rng};

pub struct Noise {
    pub input: AudioBus,
    pub output: AudioBus,
}

impl Processor for Noise {
    fn process(&mut self) {
        self.output.value = Noise::generate_random();
    }
}

impl Module for Noise {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl Noise {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
        }
    }
    fn generate_random() -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(-1.0..1.0) as f32
    }
}
