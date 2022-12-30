use crate::audio_bus::AudioBus;
use crate::processor::Processor;

pub(crate) trait Module: Processor {
    fn input(&mut self) -> &AudioBus;
    fn output(&mut self) -> &AudioBus;
}
