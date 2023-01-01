use crate::bus::Bus;
use crate::module::AudioModule;
use crate::processor::Processor;
use rand::{thread_rng, Rng};

pub struct Noise {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    amp: f32,
}

impl Processor for Noise {
    fn process(&mut self) {
        self.audio_output.value = Noise::generate_random(&self);
    }
}

impl AudioModule for Noise {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.audio_input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.audio_output
    }
}

impl Noise {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            amp: 0.1,
        }
    }
    fn generate_random(&self) -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(-self.amp..self.amp) as f32
    }
}
