use rodio::{source::Source, OutputStream};
use std::thread::sleep;
use std::time::Duration;

use crate::bus::Bus;

pub struct AudioOutput {
    input_left: Bus,
    input_right: Bus,
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            input_left: crate::bus::Bus::new(0.0),
            input_right: crate::bus::Bus::new(0.0),
        }
    }
    pub fn process(&self) {
        loop {
            println!(
                "input_right: {} input_left: {}",
                self.input_right.value, self.input_left.value
            );
            sleep(Duration::from_secs(1));
        }
    }
}
