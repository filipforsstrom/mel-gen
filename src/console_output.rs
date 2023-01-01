use crate::bus::Bus;
use crate::module::AudioModule;
use crate::processor::Processor;

pub struct ConsoleOutput {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
    output_console: String,
}

impl Processor for ConsoleOutput {
    fn process(&mut self) {
        self.output_console = self.input.value.to_string();
        println!("Output: {}", self.output_console);
    }
}

impl AudioModule for ConsoleOutput {
    fn audio_input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn audio_output(&mut self) -> &Bus<f32> {
        &mut self.output
    }
}

impl ConsoleOutput {
    pub fn new() -> Self {
        Self {
            input: Bus::<f32>::new(),
            output: Bus::<f32>::new(),
            output_console: String::new(),
        }
    }
}
