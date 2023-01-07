pub mod audio_output;
pub mod bus;
pub mod clock;
pub mod connection;
pub mod console_output;
pub mod midi_converter;
pub mod mixer;
pub mod module;
pub mod noise;
pub mod note_generator;
pub mod osc_output;
pub mod processor;
pub mod quantizer;
pub mod sample_and_hold;
pub mod scale;
pub mod system;
pub mod trigger;
pub mod wavetable;
pub mod wavetable_midi;
pub mod wavetable_oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use module::Module;
use sample_and_hold::SampleAndHold;
use wavetable::{Waveform, Wavetable};
use wavetable_midi::WavetableMidi;
use wavetable_oscillator::WavetableOscillator;

use crate::audio_output::AudioOutput;
use crate::clock::Clock;
use crate::connection::Connection;
use crate::console_output::ConsoleOutput;
use crate::noise::Noise;
use crate::processor::Processor;

const SAMPLE_RATE: f32 = 48000.0;
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
//     modules: Vec<Box<dyn Module<f32>>>,
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
    let host = cpal::default_host();

    let devices = host.devices().expect("failed to get devices");

    // move to a separate function
    // println!("Audio devices:");
    // for device in devices {
    //     println!("{}", device.name().unwrap());
    // }

    let mut device = host
        .default_output_device()
        .expect("failed to find a default output device");

    for d in devices {
        if d.name().unwrap() == "BlackHole 2ch" {
            device = d;
        }
    }

    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => {
            run::<f32>(&device, &config.into()).unwrap();
        }

        cpal::SampleFormat::I16 => {
            run::<i16>(&device, &config.into()).unwrap();
        }

        cpal::SampleFormat::U16 => {
            run::<u16>(&device, &config.into()).unwrap();
        }
    }

    // let sample_period = Duration::from_micros(1_000_000 / crate::SAMPLE_RATE as u64);
    // let mut start = Instant::now();

    // let mut connection = Connection::new(noise1.output, audio_output1.input);
    // let mut system = System::new(vec![Box::new(noise1), Box::new(audio_output1)]);

    // let mut connections = Vec::new();
    // connections.push(&mut connection);

    // let mut patch = Patch {
    //     system: &mut system,
    //     connections: connections,
    // };
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // Get the sample rate and channels number from the config
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let wavetable = Wavetable::new(WAVETABLE_SIZE);
    let mut osc1 = WavetableOscillator::new(wavetable.generate_f32(Waveform::Sawtooth));
    osc1.set_frequency(1.0);
    osc1.set_amplitude(0.01);
    let mut osc2 = WavetableOscillator::new(wavetable.generate_f32(Waveform::Sine));
    osc2.set_frequency(0.1);
    osc2.set_amplitude(0.00001);
    let mut osc_midi = WavetableMidi::new(wavetable.generate_u8(Waveform::Sawtooth));
    osc_midi.set_frequency(0.1);
    osc_midi.set_phase_low(0.0);
    osc_midi.set_phase_high(1.0);
    osc_midi.set_phase_offset(0.0);
    osc_midi.quantize_wavetable_to_scale();

    let mut noise = Noise::new();
    let mut clock = Clock::new();
    clock.set_rate(12.0);

    let mut osc_output = osc_output::OscOutput::new().unwrap();

    // Build an output stream
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                noise.process();
                clock.process();
                osc_output.process();
                osc1.process();
                osc2.process();
                osc_midi.process();

                osc_output.midi_input.value = osc_midi.midi_output.value;
                osc_output.audio_input.value = osc_midi.audio_output.value;

                // Convert into a sample
                let value: T = cpal::Sample::from::<f32>(&osc1.audio_output.value);

                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;

    // Play the stream
    stream.play()?;

    // Park the thread so our noise plays continuously until the app is closed
    std::thread::park();

    Ok(())
}
