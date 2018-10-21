pub struct SomeType {}

impl SomeType {
    pub fn new() -> SomeType {
        SomeType {}
    }

    pub fn main2() {
        let p = SomeType::new();
        p.do_nothing();
    }

    pub fn do_nothing(&self) {}
}
