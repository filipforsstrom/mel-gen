pub trait Module {
    fn new();
    fn process(&mut self);
}
