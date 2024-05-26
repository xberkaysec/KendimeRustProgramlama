fn main() {

    let mut string_1="berkay".to_string();

    string_1.clear();

    string_1.shrink_to_fit();

    println!( "string_1 boş mu: {}", string_1.is_empty()); 

}
