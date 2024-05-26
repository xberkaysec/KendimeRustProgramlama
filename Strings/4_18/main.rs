fn main() {

    let string_1 = "MERHABA".to_string();
    let string_2 = "merhaba".to_string();

    let sonuc = string_1.eq_ignore_ascii_case(&string_2);

    println!(r#" "{string_1}" ile "{string_2}" eşit mi: {sonuc} "#);
}
