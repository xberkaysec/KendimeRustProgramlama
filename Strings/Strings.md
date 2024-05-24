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
