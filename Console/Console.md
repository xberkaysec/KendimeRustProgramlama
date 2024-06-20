# Giriş

Birçok uygulama, kullanıcılarla etkileşim kurmak için konsol arayüzünü tercih eder. 
Konsolun komut satırı, olayları günlüğe kaydetmek, uygulamaları yapılandırmak, kullanıcı girdisini almak, geliştirici araçlarına erişmek ve daha fazlası için ideal bir ortam sunar. 

Tamamen komut satırı arayüzüne (CLI) dayanan konsol uygulamaları oldukça yaygındır ve grafik kullanıcı arayüzüne (GUI) uygun bir alternatiftir. 
GUI uygulamaları gibi, komut satırı arayüzleri de iyi veya kötü tasarımlara sahip olabilir. 

Unutmayın ki, bir konsol uygulaması için CLI tasarımı, bir GUI için olanlardan farklıdır. 
Bu bölüm, tasarım seçimlerine değil, konsola okuma ve yazma işlemleri için teknik yeteneklere odaklanmaktadır.

Konsola okuma ve yazma işlemleri için çeşitli komutlar mevcuttur. En yaygın olanları, println! ve print! makrolarıdır. 

Bu makrolar, konsola basit metin yazdırmak için kullanılabilir.
println! makrosu, metnin sonuna bir satır sonu eklerken, print! makrosu bunu yapmaz.

İşte basit bir örnek:

```rust
println!("Merhaba dünya!"); // Konsola "Merhaba dünya!" yazar ve bir satır sonu ekler
print!("Merhaba dünya!"); // Konsola "Merhaba dünya!" yazar, ancak satır sonu eklemez
```

Bu örnekte, println! makrosu, "Merhaba dünya!" metnini ve ardından bir satır sonu karakterini konsola yazdırır.
print! makrosu ise aynı metni yazar ancak satır sonu eklemez.

Daha ileri düzey kullanıcılara, formatlanmış metin yazdırmak, değişkenleri yazdırmak ve konsoldan giriş almak için daha gelişmiş yöntemler sunulacaktır.
