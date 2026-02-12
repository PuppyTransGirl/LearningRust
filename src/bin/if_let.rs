// EPISODE 15

#![allow(unused)]

fn main() {
    let x: Option<u32> = None;
    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }

    // if let
    if let Some(v) = x {
        // if x is a Some + get the v value
        println!("if let {v}");
    }

    // let else
    let Some(v) = x else {
        // if x is none
        // diverge - panic or return
        panic!("x is none");
    };

    println!("v = {v}");
}
