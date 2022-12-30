use crate::audio_bus::AudioBus;
use crate::processor::Processor;

pub struct Connection {
    pub input: AudioBus,
    pub output: AudioBus,
}

impl Processor for Connection {
    fn process(&mut self) {
        self.output.value = self.input.value;
    }
}

impl Connection {
    // fn new(input: Bus, output: Bus) -> Connection {
    //     Connection { input, output }
    // }

    // fn set_input(&mut self, input: Bus) {
    //     self.input = input;
    // }

    // fn set_output(&mut self, output: Bus) {
    //     self.output = output;
    // }
}
