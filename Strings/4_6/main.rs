fn main() {

    let mut alfabe = "a ".to_string();
    alfabe.push('b');

    let mut sayilar = "bir ".to_string();
    sayilar.push_str("iki");

    println!("{} | {}", alfabe, sayilar);

}
