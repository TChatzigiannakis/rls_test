pub struct Program {
    bin: Vec<u8>
}

impl Program {
    pub fn new() -> Program {
        Program { bin: Vec::new() }
    }

    pub fn main2() {
        let p = Program::new();
        p.do_nothing();
    }

    pub fn do_nothing(&self) {      
    }

    pub fn add_stuff(&mut self, mut parameters: Vec<u8>) {
        self.bin.append(&mut parameters);
    }
}
