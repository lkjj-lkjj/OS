#![no_std]
#![no_main]


#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    for i in 0..=3 {
        for _ in 0..=2 - i {
            print!(" ");
        }
        for _ in 0..=i * 2 {
            print!("*");
        }
        println!("");
    }

    for i in 0..=2 {
        for _ in 0..=i {
            print!(" ");
        }
        for _ in 0..=4 - i * 2 {
            print!("*");
        }
        println!("");
    }
    0
}