pub struct Bus<T> {
    pub value: T,
}

// impl Bus<f32> {
//     pub fn new() -> Self {
//         Self { value: 0.0 }
//     }
// }

// impl Bus<i32> {
//     pub fn new() -> Self {
//         Self { value: 0 }
//     }
// }

impl<T: Default> Bus<T> {
    pub fn new() -> Bus<T> {
        Bus {
            value: T::default(),
        }
    }
}
