use crate::bus::Bus;
use crate::module::Module;

pub struct WavetableOscillator {
    input: Bus,
    output: Bus,
    wavetable: Vec<f32>,
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    amplitude: f32,
}

impl  WavetableOscillator {
    pub fn new(wavetable: Vec<f32>) -> Self {
        Self {
            input: todo!(),
            output: todo!(),
            wavetable,
            phase: 0.0,
            frequency: 100.0,
            sample_rate: crate::SAMPLE_RATE,
            amplitude: 1.0,
        }
    }

    fn process(&mut self) {
        // Increment the phase based on the frequency and sample rate
        self.phase += self.frequency / self.sample_rate;
        // Wrap the phase back to the beginning of the wavetable if it exceeds 1
        self.phase -= self.phase.floor();

        // Interpolate the value from the wavetable using the phase
        let index1 = (self.phase * self.wavetable.len() as f32) as usize;
        let index2 = (index1 + 1) % self.wavetable.len();
        let frac = self.phase * self.wavetable.len() as f32 - index1 as f32;
        let sample = self.wavetable[index1] * (1.0 - frac) + self.wavetable[index2] * frac;

        // Scale the sample by the amplitude and convert it to an integer between 0 and 127
        self.output.value = (self.amplitude * sample * 64.0) + 64.0;
    }
}
