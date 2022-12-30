pub struct AudioBus {
    pub value: f32,
}

impl AudioBus {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }
}
