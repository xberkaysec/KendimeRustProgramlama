use std::mem;

fn main() {
    let boyut: usize = mem::size_of::<usize>();
    println!("usize'nin boyutu: {}", boyut);
}
