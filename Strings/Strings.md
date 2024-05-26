# Giriş

Bu bölüm tamamen Stringlere adanmıştır. Strings, yazılabilir ve yazılamayan karakterlerin koleksiyonlarıdır.
Rust Stringleri UTF-8 kodlama ile Unicode Standardına uyar.
Unicode, dünya çapındaki farklı dillerden ve hatta emojilerden karakterler için kod noktaları olan tek bir kod uzamını temsil eder. 
Hem aktif hem de pasif diller, örneğin hiyeroglifler, standartta yer almaktadır.
Bu, dünya çapından kullanıcılara destek sağlayan uluslararasılaştırma desteği sunar ve uygulamanızın dünya genelinden kullanıcıları desteklemesini daha kolay hale getirir.

Strings genellikle uygulamalarda görünür ve çeşitli durumlarda kullanılır, bunlar arasında uyarılar, komut satırı bilgileri, 
kullanıcı mesajları, dosya girişi, raporlama ve daha fazlası bulunmaktadır.
Birçok uygulamadaki Stringslerin yaygınlığı nedeniyle, Rust güvenli Strings sunar ve çekirdek dilde ve crates.io'da bulunan diğer kütüphanelerde çeşitli String hizmetleri sunar.

Rust'taki temel String türleri String ve str (stir olarak telaffuz edilir).

# Str

str türü, temel bir türdür ve çekirdek dilin bir parçasıdır. 
str türü, dilimin unsized ve read-only (yazılabilir olmayan) bir dilim olma özelliğine sahiptir.
str bir dilim olduğundan, genellikle bir str'yi ödünç alırsınız, &str.
Bir str, string verisinin işaretçisi ve uzunluğundan oluşan iki alanı içerir.
String literalleri, tırnaklar içinde tanımlanan ("...") str değerleridir ve programın tamamı boyunca var olur. 
Bu nedenle, bir string literalinin ömrü statiktir. 

4_1/, bir str türünü bildirme ve kullanma şeklini göstermektedir.

# String

Rust standart kütüphanesinde yer alan String veri tipi, karakter değerlerinden oluşan özel bir vektördür.
String'ler değiştirilebilir ve genişletilebilir.
Bir vektör gibi, String veri tipi üç alandan oluşur: pointer to the support array, length and capacity. 
Sport array, String'de yer alan char değerlerini içerir.
Length, String'deki karakter sayısıdır ve capacity, Sport array'in boyutudur.

Yeni String tiplerini oluşturmanın çeşitli yaklaşımları vardır. 
Çoğu zaman, String tipleri bir dize literal (yani str) kullanılarak başlatılır. 
String::from ve str::to_string gibi işlevler, bir str'yi String'e dönüştürür.

4_2/ de, from ve to_string işlevlerini kullanarak bir String literalinden iki String oluşturuyoruz.

New yapıcısını kullanarak String için boş bir dize oluşturabilirsiniz.
Genellikle, bu daha sonra metin eklenebilen değiştirilebilir bir String'dir.

4_3 te, değiştirilebilir boş bir String oluşturup ardından "Dark" dizesini ekliyoruz.

Belirtildiği gibi, String'ler özel bir vektördür - bir karakter koleksiyonudur. 
Bir String'i doğrudan bir vektorden bile oluşturabilirsiniz. 
Önce, tamsayılar olarak Unicode kod noktalarından oluşan bir vektör oluşturun. 
Her kod noktası tek bir karakteri temsil eder. 
Bir sonraki adım, vektörü from_utf8 işleviyle bir dizeye dönüştürmektir.

4_4, Unicode karakterlerinden bir String oluşturmaya bir örnektir.

# Length(uzunluk)

Belirli bir Unicode stringin uzunluğu nedir? 
Bu basit bir soru gibi görünse de karmaşık bir cevaba sahiptir. 
İlk olarak, dizesinin içinde kaç karakter ya da bayt olduğuna bağlıdır.
UTF karakterler genellikle 1 ila 4 bayt arasında değişebilir. 
ASCII karakterler, Unicode kod alanının başlangıcında bulundukları için genellikle 1 bayttır.
Ancak, kod alanının başka bir yerinde bulunan karakterler için boyutları birden fazla bayt olabilir.

İşte farklı karakter setleri için bayt boyutları:

- ASCII karakterler genellikle 1 bayt büyüklüğündedir.
- Yunan karakterler genellikle 2 bayt büyüklüğündedir.
- Çin karakterleri genellikle 3 bayt büyüklüğündedir.
- Emoji'ler genellikle 4 bayt büyüklüğündedir.

ASCII için, bayt cinsinden uzunluk ve karakter sayısı genellikle aynıdır. 
Ancak, diğer karakter setleri için bu durum farklılık gösterebilir. 
len fonksiyonu, dizenin kaç bayt içerdiğini döndürür.

Bir String içindeki karakter sayısını elde etmek için şu adımları izleyebilirsiniz: 
Stringin karakterlerini içeren bir yineleyiciyi döndüren chars fonksiyonunu kullanarak, 
yineleyici üzerindeki count fonksiyonunu çağırarak karakter sayısını elde edebilirsiniz.

4_5, bir dizenin hem bayt hem de karakter sayısını göstermektedir:

Bu örnek, farklı dillerde "Merhaba" kelimesinin hem bayt hem de karakter sayısını ekrana yazdırmaktadır.
Her dil için farklı bayt ve karakter sayılarına dikkat edilmektedir.

# String Geliştirme

String değerini genişletebilirsiniz, ancak str tipini genişletemezsiniz. 
String'in bu amaca yönelik birkaç fonksiyonu bulunmaktadır:

- push
- push_str
- insert
- insert_str

String için push fonksiyonu bir char değerini eklerken, push_str bir String ekler.

4_6/ Bir String'e Ekleme Örneği.

Matematiksel + operatörü String tipi için uygulanmıştır. 
push_str fonksiyonuna alternatif olarak, stringleri birleştirmenin başka bir yolu da + operatörünü kullanmaktır. 
+ operatörünü kullanmanın avantajı ise kolaylıktır.

4_7/ + Operatörü ile Ekleme Örneği.

Metin eklemek yerine bir string içine eklemek isteyebilirsiniz. 
Bir string içine metin eklemek için, insert fonksiyonu bir char değeri eklerken, insert_str bir string ekler. 
insert fonksiyonu için, ilk parametre örtük ve mevcut String'e işaret eder.
İkinci parametre, karakterin nereye ekleneceğini belirtir. 
Son parametre ise eklenecek karakterdir. 
insert_str fonksiyonu, son parametre olarak bir String ekler haricinde insert fonksiyonu ile aynıdır.

İşte her bir fonksiyon tanımı:

```rust
fn insert(&mut self, position: usize, ch: char)
fn insert_str(&mut self, position: usize, string: &str)
```

4_8/ Bir String İçine Ekleme Örneği.

# Capacity(kapasite)

String'ler, özel bir vektör olarak tasarlanmıştır. 
Bu vektörler, string karakterlerini depolamak için kullanılan bir destekleyici dizi(backing array) ve bir kapasiteye sahiptir.
Kapasite, destekleyici dizinin boyutunu belirtirken, uzunluk ise String'in mevcut karakter sayısını ifade eder. 
Eğer uzunluk, kapasiteyi aşarsa, destekleyici dizi genişletilerek yeniden tahsis edilmelidir. 
Ancak, backing array'in yeniden tahsis edilmesi performans açısından olumsuz etkilere neden olabilir. 
Bu yüzden, gereksiz yeniden tahsis işlemlerinden kaçınarak uygulamanın performansını artırmak önemlidir. 
String tipi, vektör veri yapısındaki gibi kapasite yönetimi için aynı fonksiyonlara sahiptir.

4_9. Capacity ve Length Karşılaştırılması

Önceki örnekte, Çince (Mandarin) "mutlu" kelimesi karakter karakter oluşturulmuştur. 
Çin dilinde "mutlu" kelimesi "快乐的" şeklindedir.
Uygulama çalıştırılırken iki kez yeniden tahsis yapılır.

Örneğin detayları şöyledir:

1. "快乐的" ifadesinin ilk karakteriyle bir string tanımlanır. Unicode'de Çince karakterler 3 byte genişliğindedir. İlk kapasite ve uzunluk 3'tür.
2. Stringe bir sonraki karakter eklenir. Uzunluk şimdi 6 olur ve kapasiteyi aşar, böylece yeniden tahsis gereklidir.
3. Stringi tamamlamak için son karakter eklenir. Uzunluk bu sefer 9 olur, tekrar kapasiteyi aşar ve başka bir yeniden tahsis gerçekleşir.

Önceki uygulamanın daha verimli olması için gereken kapasiteyi önceden bilmek önemlidir.
with_capacity fonksiyonu, bir String değeri tanımlarken kapasiteyi açıkça ayarlar. İşte fonksiyon tanımı.

```rust
fn with_capacity(capacity: usize) -> String
```

4_10/ with_capacity fonksiyonunun etkinliğini göstermek.

# String Değerine Erişim

String elemanlarına nasıl erişileceğini gösteren bir örnek ile başlayalım. 
Örnek, String'in ikinci karakterine erişmek.

```rust
let string_1 = "merhaba".to_string();
let karakter = string_1[1];
```

Ancak, yukarıdaki örnek bir derleyici hatası oluşturur:

```
error[E0277]: String türü {integer} tarafından indekslenemez
--> src\main.rs:3:19
|
3 | let karakter = string_1[1];
| ^^^^^^^^^^^ String {integer} tarafından indekslenemez
```

Hata mesajı doğru ancak altta yatan problemin tam olarak açıklanmıyor. 
Sorun, bir String'e bir indeksle erişmeye çalışmakın belirsiz olacağıdır. 
İndeks, bayt mı yoksa karakter pozisyonunu mu işaret eder? 
Bu bilgi olmadan bu ifadeyi çözmek mümkün değil veya en azından güvenli değil.
Rust'ta bir indeksle bir karaktere erişmek mümkün değildir.

Bununla birlikte, bir String dilimi kullanarak String içindeki karakterlere erişebilirsiniz. 
Başlangıç indeksi ve bitiş indeksi bayt pozisyonunu belirtir.
Slicing(dilimleme) notasyonunun sonucu ise str türündedir.

```rust
string[start indeksi..end indeksi]
```

Slice karakterin sınırlarına uymalıdır. 
Uymadığı takdirde, çalışma zamanında bir panik oluşur. A

Kod örneği başarılı bir şekilde çalışır:

```rust
let string_1 = "mutlu".to_string();
let slice = &string_1[3..=5];
println!("{:?}", slice);

// 3_11
```

# String Karakterleri

String'ler karakterlerden oluşur. Tüm karakterleri dolaşmak bazen faydalı olabilir.
Örneğin, her bir karakter üzerinde bir işlem uygulayabilir, her bir karakteri kodlayabilir,
karakterleri sayabilir veya harf "a" içeren ve çıkarılması gereken kelimeleri arayabilirsiniz, 
chars fonksiyonu, bir str değerinin karakterlerini döndüren bir iterator sağlar.

Örnek, bir kelimenin tüm karakterlerini göstermektedir.

```rust
let check_ch = "merhaba".to_string();

    for ch in check_ch.chars() {
        println!("{}", ch);
    }
}
```

Bir iterator'ün nth fonksiyonunu kullanarak belirli bir konumdaki bir karakteri gösterebilirsiniz. 
Burada, String'in üçüncü karakterini gösteriyoruz:

```rust
println!("{}", check_ch.chars().nth(2).unwrap());
```
Bir karakterin bulunduğu konumu verilen bir fonksiyon kullanarak bulabilir ve yazdırabilirsiniz. 
Bu şekilde, String üzerinde işlemler yapabilir ve karakterler üzerinde manipülasyonlar yapabilirsiniz.

```
3_12/
```

# Deref Zorlaması

Beklenen herhangi bir &str yerine ödünç alınmış bir String, &String ekleyebilirsiniz. 
Bu durumda, String, str türünün yöntemlerini devralır.
Bu, String türünün str için deref trait'ini uyguladığından mümkündür. 
Uygun şekilde, bu dönüşüm deref zorlaması olarak adlandırılır.
Tersi mümkün değildir, yani str'den String türüne dönüştürme.

4_13/, deref zorlamasını kullanan bir örneği göstermektedir.

Bu örnekte, func_deref fonksiyonunun bir &str parametresi vardır. 
Main fonksiyonunda, "Merhaba" için bir String bildiriyoruz.
Daha sonra, func_deref &String ile başarılı bir şekilde çağrılıyor.

# Biçimlendirilmiş String

Gösterişli stringler oluşturmak için format! makrosu oldukça kullanışlıdır. 
format! makrosu, dönüş biçimli bir string sağlayan print ailesiyle benzerdir.
Bu, aynı parametreleri içerir. print! ve format! makroları, std::fmt modülüne dayanır.

format! makrosunu gösteren bir örnek:

```rust
fn main() {

    let sayi_1 = 10;
    let sayi_2 = 15;

    let sonuc = format!("{sayi_1} + {sayi_2} = {sonuc}", sonuc=sayi_1+sayi_2);
    
    println!("{}", sonuc); 
    
}
```

Bu örneğin çıktısı, biçimlendirilmiş bir string olan "10 + 15 = 25" olacaktır.
Bu şekilde, format! makrosu kullanarak değişken değerlerini biçimlendirerek string oluşturabilir ve ekrana yazdırabilirsiniz. 
Bu işlem, stringleri daha okunaklı ve kullanıcı dostu hale getirmenize olanak sağlar.

# Faydalı Fonksiyonlar

Rust, String manipülasyonunu destekleyen etkileyici bir string fonksiyonlarının çeşitliliğine sahiptir. 
İşte String için daha faydalı fonksiyonlardan bazıları.

- **clear**: Bir Stringi siler ancak mevcut kapasiteyi azaltmaz. İstenirse, kapasiteyi `shrink_to_fit` fonksiyonu ile azaltabilirsiniz.
  ```rust
  fn clear(&mut self)
  ```
- **contains**: Bir Stringde deseni arar ve bulunursa true döndürür.
  ```rust
  fn contains<'a, P>(&'a self, pat: P) -> bool
  ```
- **ends_with**: Desenin String sonunda bulunması durumunda true döndürür.
  ```rust
  fn ends_with<'a, P>(&'a self, pat: P) -> bool
  ```
- **eq_ignore_ascii_case**: String deseniyle eşleşirse true döndürür. Karşılaştırma harf büyüklüğüne duyarsızdır.
  ```rust
  fn eq_ignore_ascii_case(&self, other: &str) -> bool
  ```
- **replace**: Bir Stringde bulunan deseni değiştirir. Değiştirilmiş String döndürülür.
  ```rust
  fn replace<'a, P>(&'a self, from: P, to: &str) -> String
  ```
- **split**: Bir Stringi belirtilen ayırıcıda ayrı Stringlere böler. Ayırılmış Stringleri numaralandırmak için bir yineleyici döndürür.
  ```rust
  fn split<'a, P>(&'a self, pat: P) -> Split<’a, P>
  ```
- **starts_with**: String bu desenle başlıyorsa true döndürür.
  ```rust
  fn starts_with<'a, P>(&'a self, pat: P) -> bool
  ```
- **to_uppercase**: Stringi büyük harfe dönüştürür.
  ```rust
  fn to_uppercase(&self) -> String

Her bir fonksiyonu nasıl kullanacağımızı gösterelim.

4_15, mevcut bir Stringi boşalttıktan sonra String karakter içermemektedir.
Ardından `shrink_to_fit` fonksiyonu kapasiteyi buna göre azaltır.
`is_empty` fonksiyonu, bir Stringin karakter içerip içermediğini kontrol eder.

4.16'da, contains fonksiyonu String'i "berkay" pattern(deseni) için tarar ve true döndürür.
println! makrosu sonucu gösterir. Makro içinde embedded string(gömülü dize) olduğu için format string içinde "r#" öneki ile raw string kullanılmıştır.

4_17'de, belirtilen String'in sonunda "Berkay" aranır. Sonuç true döner.

Küçük harf/küçük harf karşılaştırmaları genellikle faydalıdır. Listeleme 4.20, sadece harf büyüklüğünde farklı olan iki Dizeyi karşılaştırır. İkinci Dize, bir referans (&) kullanılarak ödünç alınmıştır. İkinci String'in konumunu özgür bırakarak, ileriye dönük bu konumda ikinci String'i kullanılamaz hale getirmez.

Listeleme 4.20. Büyük/küçük harfe duyarsız karşılaştırma

```rust
let string_4 = "BİR".to_string();
let string_5 = "bir".to_string();
let sonuc = string_4.eq_ignore_ascii_case(&string_5);
// "BİR" equals "bir": true
println!(r#""{string_4}" ile "{string_5}" eşit: {sonuc}"#);
```

Listeleme 4.21'de, replace fonksiyonu "Bob" desenini "Alice" ile değiştirir. Sonuç güncellenmiş String'i verir.

Listeleme 4.21. Bir dize içinde değiştirme

```rust
let string_6 = "Bob alışverişe gitti, sonra Bob eve döndü.".to_string();
let sonuc_string = string_6.replace("Bob", "Alice");
// Yeni dize: Alice alışverişe gitti, sonra Alice eve döndü.
println!("Yeni dize: {}", sonuc_string);
```

split fonksiyonu bir dizeyi her ayırıcıda böler. Listeleme 4.22'de, Dize boşluklarda ayraç olarak kullanılarak bölenir. Fonksiyon işlemle oluşturulan String koleksiyonu için bir yineleyici döndürür. Ardından bunları tek tek işleyebilirsiniz.

Listeleme 4.22. Boşlukları ayraç olarak kullanarak bir Dizeyi bölmek

```rust
let string_7 = "Kelime sihrini anlatır.";
let yineleyici = string_7.split(" ");
// Kelime sihrini anlatır.
for kelime in yineleyici {
    print!("{} ", kelime);
}
```

Listeleme 4.23'te, Dize "Sydney" ile başladığı için starts_with fonksiyonu true döndürür.

Listeleme 4.23. Dize önekini doğrulama

```rust
let string_8 = "Sydney manzaralıdır.".to_string();
let sonuc = string_8.starts_with("Sydney");
// "Sydney" öneki için "Sydney manzaralıdır.": true
println!(r#" "{string_8}" cümlesi için "{Sydney}" sonucu: {sonuc}"#);
```

Listenme 4.24'te, to_uppercase fonksiyonu "Harika!" kelimesini büyük harfe çevirir. Tabii ki, bir de to_lowercase fonksiyonu bulunmaktadır.

Listeleme 4.24. Bir dizeyi büyük harfe dönüştürme

```rust
let string_9 = "Harika!";
println!("{} : {}", string_9, string_9.to_uppercase());
// Harika! : HARİKA!
```
```

Bu metni Github'a Markdown formatında yükleyerek düzgün bir görünüm elde edebilirsiniz. Başka bir konuda daha yardımcı olabilir miyim?
