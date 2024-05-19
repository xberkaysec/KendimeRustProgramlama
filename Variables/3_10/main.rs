fn main() {
    
    let bakiye = -10;
    let bakiye_dusuk: bool = bakiye < 0;
    
    if bakiye_dusuk {
        println!("Bakiye: Düşük {}", bakiye);
    } else {
        println!("Bakiye: {}", bakiye);
    }

}
