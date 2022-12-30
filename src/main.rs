pub mod audio_output;
pub mod audio_bus;
pub mod clock;
pub mod connection;
pub mod console_output;
pub mod midi_converter;
pub mod module;
pub mod noise;
pub mod osc_output;
pub mod processor;
pub mod sample_and_hold;
pub mod midi_bus;
use sample_and_hold::SampleAndHold;

use crate::audio_output::AudioOutput;
use crate::audio_bus::AudioBus;
use crate::clock::Clock;
use crate::connection::Connection;
use crate::console_output::ConsoleOutput;
use crate::module::Module;
use crate::noise::Noise;
use crate::processor::Processor;

const SAMPLE_RATE: f32 = 441000.0;
const WAVETABLE_SIZE: usize = 128;

// struct Patch<'a> {
//     system: &'a mut System,
//     connections: Vec<&'a mut Connection>,
// }

// impl<'a> Processor for Patch<'a> {
//     fn process(&mut self) {
//         self.system.process();

//         for connection in &mut self.connections {
//             connection.process();
//         }
//     }
// }

// struct System {
//     modules: Vec<Box<dyn Module>>,
// }

// impl Processor for System {
//     fn process(&mut self) {
//         for module in &mut self.modules {
//             module.process();
//         }
//     }
// }

// impl System {
//     fn new(modules: Vec<Box<dyn Module>>) -> Self {
//         Self { modules }
//     }

//     fn add_module(&mut self, module: Box<dyn Module>) {
//         self.modules.push(module);
//     }
// }

fn main() {
    let mut noise = Noise::new();
    let mut clock = Clock::new();
    let mut s_h = SampleAndHold::new();
    let mut audio_output = AudioOutput::new();
    let mut console_output = ConsoleOutput::new();
    let mut osc_output = osc_output::OscOutput::new().unwrap();

    // let mut connection = Connection::new(noise1.output, audio_output1.input);
    // let mut system = System::new(vec![Box::new(noise1), Box::new(audio_output1)]);

    // let mut connections = Vec::new();
    // connections.push(&mut connection);

    // let mut patch = Patch {
    //     system: &mut system,
    //     connections: connections,
    // };

    loop {
        // patch.process();
        // thread::sleep(time::Duration::from_millis(1000)); // pause for 1000 milliseconds (1 second)

        //funkar
        osc_output.process();
        noise.process();
        clock.process();
        clock.set_rate(10.0);
        s_h.process();
        s_h.trigger.value = clock.output.value;
        s_h.input.value = noise.output.value;
        osc_output.input.value = noise.output.value;
        osc_output.trigger.value = clock.output.value;
        console_output.input.value = s_h.output.value;
        console_output.process();
    }
}
