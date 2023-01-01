use crate::bus::Bus;
use crate::processor::Processor;

pub(crate) trait AudioModule: Processor {
    fn audio_input(&mut self) -> &Bus<f32>;
    fn audio_output(&mut self) -> &Bus<f32>;
}

pub(crate) trait MidiModule: Processor {
    fn midi_input(&mut self) -> &Bus<u8>;
    fn midi_output(&mut self) -> &Bus<u8>;
}
