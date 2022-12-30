use crate::audio_bus::AudioBus;
use crate::module::Module;
use crate::processor::Processor;

pub struct ConsoleOutput {
    pub input: AudioBus,
    pub output: AudioBus,
    output_console: String,
}

impl Processor for ConsoleOutput {
    fn process(&mut self) {
        self.output_console = self.input.value.to_string();
        println!("Output: {}", self.output_console);
    }
}

impl Module for ConsoleOutput {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            output_console: String::new(),
        }
    }
}
