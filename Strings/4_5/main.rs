fn main() {
    
    let english = "Hello".to_string();
    let greek = "γεια".to_string();
    let chinese = "你好".to_string();
    let turkish = "Merhaba".to_string();

    println!(
        "İngilizce {}: Bayt {} Uzunluk {}",
        english,
        english.len(),
        english.chars().count()
    );

    println!(
        "Yunanca {}: Bayt {} Uzunluk {}",
        greek,
        greek.len(),
        greek.chars().count()
    );

    println!(
        "Çince {}: Bayt {} Uzunluk {}",
        chinese,
        chinese.len(),
        chinese.chars().count()
    );

    println!(
        "Türkçe {}: Bayt {} Uzunluk {}",
        turkish,
        turkish.len(),
        turkish.chars().count()
    );

}
