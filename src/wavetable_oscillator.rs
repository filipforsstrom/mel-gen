use crate::bus::Bus;
use crate::module::Module;
use crate::processor::Processor;

pub struct WavetableOscillator {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    wavetable: Vec<f32>,
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    amplitude: f32,
}

impl Processor for WavetableOscillator {
    fn process(&mut self) {
        self.frequency += self.audio_input.value;
        self.phase += self.frequency / self.sample_rate;
        self.phase = self.phase % 1.0;

        let table_index = (self.phase * self.wavetable.len() as f32) as usize;
        self.audio_output.value = self.amplitude * self.wavetable[table_index]
    }
}

impl Module<f32> for WavetableOscillator {
    fn input(&mut self) -> &Bus<f32> {
        &self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &self.audio_output
    }
}

impl WavetableOscillator {
    pub fn new(wavetable: Vec<f32>) -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            wavetable,
            phase: 0.0,
            frequency: 200.0,
            sample_rate: crate::SAMPLE_RATE,
            amplitude: 1.0,
        }
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
}
