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
Önce, tamsayılar olarak Unicode kod noktalarından oluşan bir vektör oluşturun. H
er kod noktası tek bir karakteri temsil eder. 
Bir sonraki adım, vektörü from_utf8 işleviyle bir dizeye dönüştürmektir.

Liste 4.4, Unicode karakterlerinden bir String oluşturmaya bir örnektir.

Kod Listesi 4.4. Unicode Karakterlerini Bir Dizeye Dönüştürme

let vec_1 = vec![65, 114, 107, 97, 110, 115, 97, 115];
let string_1 = String::from_utf8(vec_1).unwrap();


Bu örnekte, "Arkansas" için kod noktaları bir vektör içinde yer almaktadır. Örneğin, 65 kod noktası Unicode tablosundaki 'A' karakteridir. from_utf işlevi daha sonra vektörü bir String'e dönüştürür.
