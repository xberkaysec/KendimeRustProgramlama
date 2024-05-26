fn main() {

    let string_1 = "Bu gün hava çok güzel.";

    let yineleyici = string_1.split(" ");
    // Kelime sihrini anlatır.
    
    for kelime in yineleyici {
        print!("{} ", kelime);
    }
}
