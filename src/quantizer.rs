use crate::{bus::Bus, module::Module, processor::Processor};

pub struct Quantizer {
    pub audio_input: Bus<f32>,
    pub audio_output: Bus<f32>,
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
    previous_note: u8,
    wavetable_cycle: u8,
}

impl Processor for Quantizer {
    fn process(&mut self) {
        self.previous_note = self.midi_output.value;

        // let scale = [0, 2, 3, 5, 7, 8, 10];
        let scale = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let octave = self.midi_input.value / 12;
        let within_octave = self.midi_input.value % 12;

        let mut nearest_distance = std::u8::MAX;
        let mut nearest_note = 0;
        for (i, &note) in scale.iter().enumerate() {
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

        let note = octave * 12 + scale[nearest_note];

        if note != self.previous_note || self.wavetable_cycle > 127 {
            self.audio_output.value = 1.0;
        } else {
            self.audio_output.value = -1.0;
        }

        self.wavetable_cycle += 1;
        if self.wavetable_cycle > 127 {
            self.wavetable_cycle = 0;
        }

        self.midi_output.value = note
    }
}

impl Module<f32> for Quantizer {
    fn input(&mut self) -> &Bus<f32> {
        &mut self.audio_input
    }

    fn output(&mut self) -> &Bus<f32> {
        &mut self.audio_output
    }
}

impl Module<u8> for Quantizer {
    fn input(&mut self) -> &Bus<u8> {
        &mut self.midi_input
    }

    fn output(&mut self) -> &Bus<u8> {
        &mut self.midi_output
    }
}

impl Quantizer {
    pub fn new() -> Self {
        Self {
            audio_input: Bus::<f32>::new(),
            audio_output: Bus::<f32>::new(),
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
            previous_note: 0,
            wavetable_cycle: 0,
        }
    }
}
