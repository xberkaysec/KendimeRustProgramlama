fn main() {

    let numbers = [("192", 192), ("168", 168), ("1", 1), ("1", 1)];

    println!("{:7}{:10}", "Değer", "Binary");
    println!("{:7}{:10}", "====", "=========");
    
    for (isim, deger) in numbers {
        println!("{:7}{:07b}", isim, deger);
    }
}
