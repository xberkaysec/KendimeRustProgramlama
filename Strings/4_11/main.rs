fn main() {
 
    let string_1 = "merhaba".to_string();

    let slice = &string_1[3..=4]; // ha
    
    println!("{:?}", slice); 
}
