use std::net::UdpSocket;

use rosc::{encoder, OscMessage, OscPacket, OscType};

use crate::{bus::Bus, module::Module, processor::Processor};

pub struct OscOutput {
    pub midi_input: Bus<u8>,
    pub midi_output: Bus<u8>,
    pub trigger: Bus<f32>,
    to_port: u16,
    socket: UdpSocket,
}

impl Processor for OscOutput {
    fn process(&mut self) {
        if self.trigger.value > 0.0 {
            self.send_osc(self.midi_input.value as i32).unwrap();
        }
    }
}

impl Module<u8> for OscOutput {
    fn input(&mut self) -> &Bus<u8> {
        &mut self.midi_input
    }

    fn output(&mut self) -> &Bus<u8> {
        &mut self.midi_output
    }
}

impl OscOutput {
    pub fn new() -> Result<Self, String> {
        let socket = match UdpSocket::bind(("127.0.0.1:1235")) {
            Ok(socket) => socket,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            midi_input: Bus::<u8>::new(),
            midi_output: Bus::<u8>::new(),
            trigger: Bus::<f32>::new(),
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
