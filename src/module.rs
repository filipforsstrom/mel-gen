use crate::bus::Bus;
use crate::processor::Processor;

pub(crate) trait Module<T>: Processor {
    fn input(&mut self) -> &Bus<T>;
    fn output(&mut self) -> &Bus<T>;
}