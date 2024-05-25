fn main() {
 
    let mut string_1 = String::with_capacity(9);

    string_1.push('快');

    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 9 Uzunluk 9

    string_1.push('乐');

    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 9 Uzunluk 9
    
    string_1.push_str("的");

    println!("Kapasite {} Uzunluk {}", string_1.capacity(), string_1.len()); // Kapasite 9 Uzunluk 9

}
