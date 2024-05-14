use std::f64::consts;

fn main() {
    let yarı_cap = 4.234;
    let cap = 2.0 * yarı_cap;
    let alan = consts::PI * yarı_cap;

    println!("{} {} {}", yarı_cap, cap, alan);

}
