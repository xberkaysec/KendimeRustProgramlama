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
