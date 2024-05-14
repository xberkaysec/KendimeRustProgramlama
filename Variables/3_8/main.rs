fn main() {

    let sayi: f64 = 10.0;
    let sonsuzluk: f64 = f64::INFINITY;

    if sayi > sonsuzluk {
        println!("Sayı sonsuzluktan büyük.");
    } else {
        println!("Sayı sonsuzluktan küçük veya eşit.");
    }

}
