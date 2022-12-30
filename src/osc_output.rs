use std::{error::Error, io::ErrorKind, net::UdpSocket};

use rosc::{encoder, OscMessage, OscPacket, OscType};

use crate::{audio_bus::AudioBus, module::Module, processor::Processor};

pub struct OscOutput {
    pub input: AudioBus,
    pub output: AudioBus,
    pub trigger: AudioBus,
    to_port: u16,
    socket: UdpSocket,
}

impl Processor for OscOutput {
    fn process(&mut self) {
        if self.trigger.value > 0.0 {
            self.output.value = self.input.value;
            self.send_osc(((self.input.value + 1.0) * 127.0 / 2.0) as i32);
        }
    }
}

impl Module for OscOutput {
    fn input(&mut self) -> &AudioBus {
        &mut self.input
    }

    fn output(&mut self) -> &AudioBus {
        &mut self.output
    }
}

impl OscOutput {
    pub fn new() -> Result<Self, String> {
        let socket = match UdpSocket::bind(("127.0.0.1:1235")) {
            Ok(socket) => socket,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            input: AudioBus::new(),
            output: AudioBus::new(),
            trigger: AudioBus::new(),
            to_port: 666,
            socket,
        })
    }

    pub fn send_osc(&self, value: i32) -> Result<(), String> {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/note".to_string(),
            args: vec![OscType::Int(value)],
        }))
        .unwrap();

        let result = self.socket.send_to(&msg_buf, ("127.0.0.1", self.to_port));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
