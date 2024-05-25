fn main() {

    let mut string_1 = '快'.to_string(); // a

    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 3 Uzunluk 3
    
    string_1.push('乐'); // b
    
    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 8 Uzunluk 6
    
    string_1.push_str("的"); // c
    
    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 16 Uzunluk 9

}
