fn main() {

    let n = -1.0;
    let sonuc = f64::sqrt(n);

    if f64::NAN != sonuc {
        println!("Karekök : {}", sonuc);
    } else {
        println!("NaN!");
    }

}
