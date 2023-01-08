use crate::bus::Bus;
use crate::module::Module;
use crate::processor::Processor;
use crate::wavetable::Waveform;

pub struct WavetableMidi {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
    waveform: Waveform,
    wavetable: Vec<u8>,
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    phase_range: (f32, f32),
    phase_low: f32,
    phase_high: f32,
    phase_offset: f32,
    scale: Vec<u8>,
    previous_table_index: usize,
}

impl Processor for WavetableMidi {
    fn process(&mut self) {
        self.midi_output.value = self.next_sample();
    }
}

impl Module<f32> for WavetableMidi {
    fn input(&mut self) -> &Bus<f32> {
        &self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &self.audio_output
    }
}

impl Module<u8> for WavetableMidi {
    fn input(&mut self) -> &Bus<u8> {
        &self.midi_input
    }

    fn output(&mut self) -> &Bus<u8> {
        &self.midi_output
    }
}

impl WavetableMidi {
    pub fn new(wavetable: Vec<u8>) -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
            waveform: Waveform::Sine,
            wavetable,
            phase: 0.0,
            frequency: 200.0,
            sample_rate: crate::SAMPLE_RATE,
            phase_range: (0.0, 1.0),
            phase_offset: 0.0,
            phase_low: 0.0,
            phase_high: 1.0,
            scale: [0, 2, 3, 5, 7, 8, 10].to_vec(),
            previous_table_index: 0,
        }
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    pub fn set_phase_low(&mut self, phase_low: f32) {
        self.phase_low = phase_low;
        self.update_phase_range();
    }
    pub fn set_phase_high(&mut self, phase_high: f32) {
        self.phase_high = phase_high;
        self.update_phase_range();
    }
    pub fn set_phase_offset(&mut self, phase_offset: f32) {
        self.phase_offset = phase_offset;
        self.update_phase_range();
    }
    fn next_sample(&mut self) -> u8 {
        self.frequency += self.audio_input.value;
        self.phase += self.frequency / self.sample_rate;
        self.phase = self.phase % self.phase_range.1;

        if self.phase < self.phase_range.0 {
            self.phase = self.phase_range.0;
        } else if self.phase > self.phase_range.1 {
            self.phase = self.phase_range.1;
        }

        let table_index = (self.phase * self.wavetable.len() as f32) as usize;

        if table_index != self.previous_table_index {
            self.audio_output.value = 1.0;
        } else {
            self.audio_output.value = -1.0;
        }

        self.previous_table_index = table_index;
        self.wavetable[table_index]
    }
    fn update_phase_range(&mut self) {
        self.phase_range = (
            (self.phase_low + self.phase_offset).min(0.9).max(0.0),
            (self.phase_high + self.phase_offset).min(1.0).max(0.1),
        );
    }
    pub fn update_wavetable(&mut self) {
        let wavetable_to_change = self.wavetable.clone();
        let mut new_wavetable = Vec::new();
        for value in wavetable_to_change {
            let new_value = self.quantize_note_to_scale(value);
            new_wavetable.push(new_value);
        }
        self.wavetable = new_wavetable;
    }
    fn quantize_note_to_scale(&mut self, input_note: u8) -> u8 {
        let octave = input_note / 12;
        let within_octave = input_note % 12;

        let mut nearest_distance = std::u8::MAX;
        let mut nearest_note = 0;
        for (i, &note) in self.scale.iter().enumerate() {
            let distance = if note > within_octave {
                note - within_octave
            } else {
                within_octave - note
            };
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_note = i;
            }
        }

        let note = octave * 12 + self.scale[nearest_note];
        note
    }
}
