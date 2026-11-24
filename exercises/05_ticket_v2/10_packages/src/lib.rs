use std::fmt::Write as _;

pub fn hello_world() {
    let mut msg: String = "Hello world".to_string();
    writeln!(msg);
}
