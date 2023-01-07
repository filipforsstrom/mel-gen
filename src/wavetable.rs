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

    pub fn generate_f32(&self, waveform: Waveform) -> Vec<f32> {
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

    pub fn generate_u8(&self, waveform: Waveform) -> Vec<u8> {
        let mut wavetable = vec![0.0; self.size];

        for (i, sample) in wavetable.iter_mut().enumerate() {
            let phase = (i as f32) / (self.size - 1) as f32;

            match waveform {
                Waveform::Sine => *sample = (2.0 * PI * phase).sin(),
                // BUG - outputs 0-63, should be 0-127
                Waveform::Triangle => *sample = 2.0 * (((2.0 * phase) % 1.0) - 0.5).abs() - 1.0,
                Waveform::Square => *sample = if (2.0 * phase) % 1.0 < 0.5 { 1.0 } else { -1.0 },
                Waveform::Sawtooth => *sample = 2.0 * (phase % 1.0) - 1.0,
            }
        }

        // Map the f32 values to u8 values in the range 0 to 127
        wavetable
            .into_iter()
            .map(|x| ((x + 1.0) * 0.5 * 255.0 / 2.0) as u8)
            .collect()
    }
}
