use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {

    // Decimal türünden iki sayı oluşturuluyor
    let mut sayi1 = Decimal::from_str("1.12345").unwrap();
    let mut sayi2 = dec!(1.12345); 

    // Sayı1'i, 1 ondalık basamağa yuvarlayarak değiştiriyor...
    let yuvarlanmis_sayi1 = sayi1.round_dp(1);

    // Sayı2'yi, 2 ondalık basamağa yuvarlayarak değiştiriyor...

    let yuvarlanmis_sayi2 = sayi2.round_dp(2);

    // Yuvarlanmış sayıyı ekrana yazdırma
    println!("{}", yuvarlanmis_sayi1);
    println!("{}", yuvarlanmis_sayi2);

}
