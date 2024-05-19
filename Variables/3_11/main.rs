// Pointers 

use std::env::consts;

fn main() {
    
    let a: i32 = 10;
    // 10
    let b: i32 = 20;
    // 20
    let c: i32 = 30;
    // 30

    let pa = &a as *const i32;
    // 0x36d52ff4d0
    let pb = &b as *const i32;
    // 0x36d52ff4d4 
    let pc: *const i32 = &c as *const i32;
    // 0x7fff149df27c
    
    println!("{:?} {:?} {:?}", pa, pb, pc);

}
