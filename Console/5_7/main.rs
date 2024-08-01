fn main() {
    let numbers = [("Bir", 100), ("İki", 200), ("Üç", 300)];

    println!("{:7}{:10}", "Metin", "Değer");
    println!("{:7}{:10}", "====", "=====");
    
    for (isim, deger) in numbers {
        println!("{:7}{:<10}", isim, deger);
    }
}
