fn main() {
    
    let meyve_bahcesi = 20;
    
    // portakal, meyve bahçesini referanslayarak başlatıldı
    let portakallar: &i32 = &meyve_bahcesi;

    // elmalar, sabit bir değere referans göstererek başlatıldı
    let elmalar: &i32 = &10;

    // Referanslara ayrıştırma işlemi yapmamıza gerek yok
    let sepet = *portakallar + *elmalar;
    
    println!("{}", sepet);
  
}
