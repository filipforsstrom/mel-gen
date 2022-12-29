use rand::{thread_rng, Rng};
use std::{thread, time};

trait Processor {
    fn process(&mut self);
}

trait Module: Processor {
    fn input(&mut self) -> &Bus;
    fn output(&mut self) -> &Bus;
}

struct AudioOutput {
    input: Bus,
    output: Bus,
    output_left: String,
    output_right: String,
}

impl Processor for AudioOutput {
    fn process(&mut self) {
        self.output_left = self.input.value.to_string();
        self.output_right = self.input.value.to_string();
        println!(
            "AudioOutput: Left={}, Right={}",
            self.output_left, self.output_right
        );
    }
}

impl Module for AudioOutput {
    fn input(&mut self) -> &Bus {
        &mut self.input
    }

    fn output(&mut self) -> &Bus {
        &mut self.output
    }
}

struct Noise {
    input: Bus,
    output: Bus,
}

impl Processor for Noise {
    fn process(&mut self) {
        self.output.value = Noise::generate_random_double();
    }
}

impl Module for Noise {
    fn input(&mut self) -> &Bus {
        &mut self.input
    }

    fn output(&mut self) -> &Bus {
        &mut self.output
    }
}

impl Noise {
    fn generate_random_double() -> f64 {
        let mut rng = thread_rng();
        rng.gen_range(-1.0..1.0)
    }
}

// #[derive(Copy, Clone)]
struct Bus {
    value: f64,
}

struct Connection {
    input: Bus,
    output: Bus,
}

impl Processor for Connection {
    fn process(&mut self) {
        self.output.value = self.input.value;
    }
}

impl Connection {
    fn new(input: Bus, output: Bus) -> Connection {
        Connection { input, output }
    }

    fn set_input(&mut self, input: Bus) {
        self.input = input;
    }

    fn set_output(&mut self, output: Bus) {
        self.output = output;
    }
}

struct Patch<'a> {
    system: &'a mut System,
    connections: Vec<&'a mut Connection>,
}

impl<'a> Processor for Patch<'a> {
    fn process(&mut self) {
        self.system.process();

        for connection in &mut self.connections {
            connection.process();
        }
    }
}

struct System {
    modules: Vec<Box<dyn Module>>,
}

impl Processor for System {
    fn process(&mut self) {
        for module in &mut self.modules {
            module.process();
        }
    }
}

fn main() {
    let mut noise1 = Noise {
        input: Bus { value: 0.0 },
        output: Bus { value: 0.0 },
    };

    let mut audio_output1 = AudioOutput {
        input: Bus { value: 0.0 },
        output: Bus { value: 0.0 },
        output_left: String::new(),
        output_right: String::new(),
    };

    let mut connection = Connection::new(noise1.output, audio_output1.input);

    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    modules.push(Box::new(noise1));
    modules.push(Box::new(audio_output1));

    let mut system = System { modules };

    let mut connections = Vec::new();
    connections.push(&mut connection);

    let mut patch = Patch {
        system: &mut system,
        connections: connections,
    };

    loop {
        patch.process();
        thread::sleep(time::Duration::from_millis(1000)); // pause for 1000 milliseconds (1 second)

        //funkar
        // noise1.process();
        // audio_output1.input.value = noise1.output.value;
        // audio_output1.process();
    }
}
