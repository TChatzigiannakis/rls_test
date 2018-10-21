mod some_type;

use some_type::SomeType;

fn main() {
    let p = SomeType::new();
    p.do_nothing();

    SomeType::main2();
}
