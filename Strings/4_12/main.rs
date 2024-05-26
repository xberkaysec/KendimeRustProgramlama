fn main() {
 
    let check_ch = "merhaba".to_string();

    for ch in check_ch.chars() {
        println!("{}", ch);
    }

    println!("-----------------------");
    
    println!("{}", check_ch.chars().nth(2).unwrap());

}
