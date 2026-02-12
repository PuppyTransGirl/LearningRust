// EPISODE 17

// super::foo = ../foo
use super::foo;

pub fn call_foo() {
    foo::print();
}

pub fn print() {
    f();
    println!("my");
}

fn f() {}

// Nested module
pub mod a;
