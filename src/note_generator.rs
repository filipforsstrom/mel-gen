use crate::bus::Bus;
use crate::module::Module;
use crate::processor::Processor;

pub struct NoteGenerator {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
    active_wavetable: Vec<u8>,
    wavetable: Vec<u8>,
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    amplitude: f32,
    note_high: u8,
    note_low: u8,
    previous_note: u8,
    index: usize,
}

impl Processor for NoteGenerator {
    fn process(&mut self) {
        if self.audio_input.value > 0.0 {
            self.previous_note = self.midi_output.value;

            let mut note = self.midi_output.value;

            for _ in 0..127 {
                note = self.next_sample();
                if note != self.previous_note {
                    break;
                }
            }

            self.midi_output.value = note;
        }
    }
}

impl Module<f32> for NoteGenerator {
    fn input(&mut self) -> &Bus<f32> {
        &self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &self.audio_output
    }
}

impl Module<u8> for NoteGenerator {
    fn input(&mut self) -> &Bus<u8> {
        &self.midi_input
    }

    fn output(&mut self) -> &Bus<u8> {
        &self.midi_output
    }
}

impl NoteGenerator {
    pub fn new(wavetable: Vec<u8>) -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
            active_wavetable: wavetable.clone(),
            wavetable,
            phase: 0.0,
            frequency: 200.0,
            sample_rate: crate::SAMPLE_RATE,
            amplitude: 1.0,
            note_high: 127,
            note_low: 0,
            previous_note: 0,
            index: 0,
        }
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
    pub fn set_note_range(&mut self, note_low: u8, note_high: u8) {
        self.note_low = note_low;
        self.note_high = note_high;

        let wavetable = self.wavetable.clone();

        let input_low = wavetable.iter().min().unwrap();
        let input_high = wavetable.iter().max().unwrap();
        let output_low = note_low;
        let output_high = note_high;

        let mut modified_wavetable = Vec::new();
        for value in wavetable.iter() {
            let modified_value = ((*value - *input_low) / (*input_high - *input_low))
                * (output_high - output_low)
                + output_low;
            modified_wavetable.push(modified_value);
        }

        self.active_wavetable = modified_wavetable;
    }

    fn next_sample(&mut self) -> u8 {
        self.index += 1;
        self.index %= self.active_wavetable.len();

        self.active_wavetable[self.index]
    }
}
