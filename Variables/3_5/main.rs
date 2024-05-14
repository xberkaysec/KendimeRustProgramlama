fn main() {
    
    let deger = i8::MAX;

    let sonuc = deger.overflowing_add(1); 

    println!("{:?}", sonuc);

}
