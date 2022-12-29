use std::f32::consts::PI;

pub enum Waveform {
    Sine,
    Triangle,
    Square,
    Sawtooth,
}

pub struct Wavetable {
    size: usize,
}

impl Wavetable {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn generate(&self, waveform: Waveform) -> Vec<f32> {
        let mut wavetable = vec![0.0; self.size];

        for (i, sample) in wavetable.iter_mut().enumerate() {
            let phase = (i as f32) / (self.size - 1) as f32;

            match waveform {
                Waveform::Sine => *sample = (2.0 * PI * phase).sin(),
                Waveform::Triangle => *sample = 2.0 * (((2.0 * phase) % 1.0) - 0.5).abs() - 1.0,
                Waveform::Square => *sample = if (2.0 * phase) % 1.0 < 0.5 { 1.0 } else { -1.0 },
                Waveform::Sawtooth => *sample = 2.0 * (phase % 1.0) - 1.0,
            }
        }

        wavetable
    }
}
