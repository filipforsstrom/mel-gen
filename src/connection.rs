use crate::bus::Bus;
use crate::processor::Processor;

pub struct Connection {
    pub input: Bus<f32>,
    pub output: Bus<f32>,
}

impl Processor for Connection {
    fn process(&mut self) {
        self.output.value = self.input.value;
    }
}

impl Connection {
    fn new(input: Bus<f32>, output: Bus<f32>) -> Connection {
        Connection { input, output }
    }

    fn set_input(&mut self, input: Bus<f32>) {
        self.input = input;
    }

    fn set_output(&mut self, output: Bus<f32>) {
        self.output = output;
    }
}
