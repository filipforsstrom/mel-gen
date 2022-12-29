use crate::wavetable_oscillator::WavetableOscillator;

pub struct System {
    pub modules: Vec<WavetableOscillator>,
}

impl System {
    pub fn process(&mut self) {
        for module in &mut self.modules {
            module.process();
        }
    }
}
