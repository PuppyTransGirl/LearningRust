// EPISODE 17

pub fn print() {
    println!("a");
}

#[derive(Debug)]
pub struct S {
    pub id: u32,
    pub name: String,
}

use super::super::foo;

pub fn call_foo() {
    foo::print();
}
