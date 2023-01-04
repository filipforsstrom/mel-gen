use std::time::Duration;

use crate::bus::Bus;
use crate::module::Module;
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
        std::thread::sleep(Duration::from_millis(100));
    }
}

impl Module<f32> for ConsoleOutput {
    fn input(&mut self) -> &Bus<f32> {
        &mut self.input
    }

    fn output(&mut self) -> &Bus<f32> {
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
