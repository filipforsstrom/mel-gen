use crate::{module::Module, processor::Processor};

pub struct System<T> {
    modules: Vec<Box<dyn Module<T>>>,
}

impl Processor for System<f32> {
    fn process(&mut self) {
        for module in &mut self.modules {
            module.process();
        }
    }
}
