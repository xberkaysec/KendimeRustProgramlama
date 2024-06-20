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

# Print

print! ve println! makroları, konsolda bilgi görüntülemek için sıklıkla kullanılır. 
Her iki makro da biçimlendirilmiş bir dizeyi standart çıktı akışına (stdout) ekler.
println! makrosu biçimlendirilmiş çıktıya bir satır sonu eklerken, print! makrosu bunu yapmaz.

Her makronun ilk parametresi bir biçim dizesi ve bir dize sabitidir.
Biçim dizesi, {} karakterleri olan yer tutucular içerebilir. 
print! makrolarının kalan parametreleri, biçim dizesindeki yer tutucular için ikame argümanlarıdır.

print! ve println! makroları değişken sayıda argüman alabilir (variadic).
print! makrosunun en az bir argümanı, yani format string olmalıdır.
println! makrosunun argümanı olmayabilir ve yalnızca bir satır sonu görüntüler.

{} yer tutucuları(placeholders), Display özelliğini uygulayan, herkese açık türler için ayrılmıştır.
Herkese açık türler genellikle bilinen bir temsile sahiptir. 
Standart kitaplıktaki birçok ilkel tür, örneğin integer ve float sayılar, herkese açık türler olarak kabul edilir ve Display özelliğini uygular. 
Struct gibi diğer türler, Display özelliğini uygulamayabilir. Bu türler {} yer tutucuları içinde kullanılamaz.

5_1 bir örnektir.

Bu örnekte, println! makrosunun biçim dizesi üç yer tutucuya sahiptir. 
Yer tutucular, iki değişken değeri ve bir hesaplama ile değiştirilir.

print! ve println! makroları, konsol uygulamaları için çıktı üretmek için güçlü ve kullanışlı araçlardır.

# Pozisyonel Argümanlar

Format strings, pozisyonel argümanlar için yer tutucular içerebilir. 
Bir pozisyonel argümanı belirtmek için bir index kullanırsınız: {index}. 
İndex sıfırdan başlar ve usize türündedir.

Pozisyonel argümanları kullanmanın temel faydası, parametrelerin sırasız bir şekilde görüntülenmesine olanak tanır.
5.2'de verilen örnekte, formatted string argümanları ters sırada görüntülenir.

Bir format string de, yer tutucuların türlerini karıştırabilirsiniz: pozisyonel olmayan ve pozisyonel. 
Ancak, pozisyonel olmayan argümanlar önce değerlendirilir. 

5.3'te gösterilen kaynak kod, her iki argüman türünün de kullanımını göstermektedir.

# Variable(Değişken) Argümanlar

Format stringlerdeki yer tutucular aynı zamanda değişkenlere de referans olabilir. 
Bir değişken argüman, bir tür pozisyonel argümandır. 
İlgili değişken, kapsam içinde olmalı ve belirtilen zamanda görünür olmalıdır. 
5.4'te, ilk, iki, üc, dört ve sonuc değişkenleri, biçim dizisindeki yer tutucular içinde referans alınmıştır.

# Named(isimli) Argümanlar

print! makroları isimli argümanları kullanabilir.
Sözdizimi isim=değer şeklindedir. 
Bu argümanlar daha sonra format stringteki yer tutucular içinde kullanılabilir. 
5.5, 'sonuc' isimli bir argümandır ve format string son yer tutucusu içinde kullanılmaktadır.

Her isimli argümanlar ve konumsal argümanlar, format stringdeki yer tutucular içinde görünebilir. 
Ancak, konumsal argümanlar parametre listesinde isimli argümanları takip edemez.
5.6, konumsal argümanlardan önce ad, bir isimli argümandır ve println! makrosunda başka herhangi bir konumsal argümanlardan önce gelir.
