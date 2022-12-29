use crate::connection::Connection;
use crate::system::System;

pub struct Patch {
    system: System,
    connections: Vec<Connection>,
}

impl Patch {
    pub fn new(system: System, connections: Vec<Connection>) -> Self {
        Self {
            system,
            connections,
        }
    }

    pub fn process(&mut self) {
        self.system.process();
        for connection in &self.connections {
            connection.process();
        }
    }
}
