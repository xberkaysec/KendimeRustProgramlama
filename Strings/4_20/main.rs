fn main() {

    let string_7 = "Bu gün hava çok güzel.";

    let yineleyici = string_7.split(" ");
    // Kelime sihrini anlatır.
    
    for kelime in yineleyici {
        print!("{} ", kelime);
    }
}
