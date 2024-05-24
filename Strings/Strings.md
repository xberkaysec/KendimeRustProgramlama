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

