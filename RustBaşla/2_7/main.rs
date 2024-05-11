fn main() {
    // İlk argümanı görüntüle
    if let Some(arg) = std::env::args().nth(0) {
        println!("{}", arg);
    }
    
    // Argümanları teker teker görüntüle
    for arg in std::env::args() {
        println!("{}", arg);
    }
    
    // Tüm argümanları topla ve görüntüle
    let all_arg: String = std::env::args().collect();
    println!("{}", all_arg);
}
