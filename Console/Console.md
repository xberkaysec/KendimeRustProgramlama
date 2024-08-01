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

# Padding, Alignment, and Precision (Dolgu, Hizalama ve Hassasiyet)

Format Stringde, placeholders'lerin padding'ini, hizalamasını veya sayısal hassasiyetini ayarlayabilirsiniz.
Bu, profesyonel görünümlü görüntüler ve raporlar oluşturmak için harika bir özelliktir.
Format spesifikasyonunu, placeholders'dan sonra gelen : (iki nokta) karakteriyle ince ayar yapabilirsiniz, yani {:format} şeklinde.

Placeholders padding'ini veya sütun genişliğini ayarlamak için {:width} sözdizimini kullanırsınız. 
Sütun içinde, sayısal değerler için varsayılan hizalama sağa hizalamadır.
Metinler için ise varsayılan hizalama sola doğrudur.
Varsayılan hizalamayı şu karakterlerle geçersiz kılabilirsiniz:

```
- > Sağ hizalama
- < Sol hizalama
- ^ Ortaya hizalama
```
 
5_7 kod örneği, bir placeholders genişliğini ve hizalamasını nasıl tanımlayacağınızı göstermektedir.

Bu bölümde, yazdırılan verilerin düzgün bir şekilde hizalanması ve biçimlendirilmesi üzerinde durulmaktadır.
Aşağıda, iki sütunlu bir rapor örneği verilmiştir.
İlk sütun yedi karakter genişliğinde metin içerirken, ikinci sütun on karakter genişliğinde sayılardan oluşur. 
İlk sütun için varsayılan hizalama sola, ikinci sütun için ise açıkça sola hizalama ayarlanmıştır.

```
Text   Value
====   =====
Bir    100
İki    200
Üç     300
```

Decimal sayılar için placeholdersda hassasiyet ekleyebilirsiniz. 
Bu, ondalık noktadan sonraki basamak sayısını kontrol etmenizi sağlar. 
Placeholdersda, padding'ten sonra ondalık hassasiyeti belirtebilirsiniz.
Sözdizimi şu şekildedir: padding.precision. Eğer padding yoksa, sözdizimi sadece .precision şeklindedir.
Tam sayılar için yer tutucudaki hassasiyet göz ardı edilir.


5_8 kodda iki ondalık sayı gösterilmektedir.
İlk sayı, on karakter genişliğinde iki ondalık basamakla,
ikinci sayı ise varsayılan genişlikte 6 ondalık basamakla görüntülenmektedir.

Ayrıca, hassasiyeti veya genişliği `$` karakteri ile parametreleştirebilirsiniz. 
Format stringte $ karakterinin önüne, parametre listesindeki argümanın konumunu belirtmelisiniz.
Aşağıdaki kodda, println! makrosu iki ondalık sayıyı göstermektedir.
$ karakterlerinin yerleşimine dikkat edin; her biri hassasiyet tanımını bir argüman olarak belirlemektedir.
Her iki placeholders için de hassasiyet sıfır olarak seçilmiş ve iki basamak olarak ayarlanmıştır.

Kod örneği, 5_9.

Bu kodda 1 ve 2 argümanları kullanılarak her iki yer tutucunun da hassasiyeti iki basamak olarak ayarlanmıştır.
Bu özellikler sayesinde verilerinizi daha okunabilir ve profesyonel bir şekilde sunabilirsiniz.

İkisinin karşılıklı kullanımı örnek, 6_1.

Rust dilinde println! makrosu ile formatlama yaparken, yer tutucuların hem padding (boşluk) hem de precision (hassasiyet) değerlerini parametreleştirmek mümkündür. 
Bu, formatlama işlemini daha dinamik hale getirir.


Örnek,

```rust
println!("Result: {2:<0$.1$} {3:.1$}", 10, 2, f1, f2);
```

Bu satırda, placeholders şu şekilde kullanılmıştır:

- {2:<0$.1$}: Burada 2 numarası, f1 değişkenini temsil eder. < işareti, değerin sola yaslanmasını sağlar.
-  0$, padding (boşluk) için kullanılacak olan ilk argümanı belirtir ve 1$, ondalık hassasiyeti belirten ikinci argümanı temsil eder.
  
- {3:.1$}: Burada 3 numarası, f2 değişkenini temsil eder. 1$, ondalık hassasiyetini belirten ikinci argümanı kullanır.

- **Padding (Boşluk)**: 0$ ifadesi, ilk argümanın (bu durumda 10) kullanılacağını belirtir.
- Bu, f1 değerinin toplam genişliğini belirler. Yani, f1 değeri bu genişliğe göre sola yaslanır.
  
- **Precision (Hassasiyet)**: 1$ ifadesi, ikinci argümanın (bu durumda 2) kullanılacağını belirtir. Bu, ondalık basamak sayısını belirler.


Diyelim ki,

```rust
// f1 = 1.2345 ve f2 = 3.123456.
// Yukarıdaki kod şöyle çalışır:

let f1 = 1.2345;
let f2 = 3.123456;
println!("Result: {2:<0$.1$} {3:.1$}", 10, 2, f1, f2);
```

Burada:
- 10 boşluk olarak kullanılır; yani f1 değeri toplamda 10 karakter genişliğinde olacak şekilde yazdırılır.
- 2 ise ondalık basamak sayısını belirler; yani f2 değeri 2 ondalık basamakla yazdırılır.

Çıktı,

```
Result: 1.23      3.12
```
