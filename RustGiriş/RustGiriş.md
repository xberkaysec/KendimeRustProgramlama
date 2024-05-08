# Giriş

Rust, güvenli, güvenilir ve ölçeklenebilir uygulamalar oluşturmak için genel amaçlı bir dil olarak kullanılır.
Dil, birkaç programlama paradigmasından özellikler içerir.
Başlangıçta bir sistem programlama dili olarak tasarlanan Rust, daha sonradan sistem programlama, web hizmetleri, masaüstü uygulamalar,
gömülü sistemler ve daha fazlası gibi çeşitli uygulama türlerinin yaratılmasında kullanılabilen daha çok yönlü bir dil olarak ortaya çıkmıştır.

Rust sözdizimi C ve C++ dillerine dayansa da, diğer C tabanlı dillerle benzerlik genellikle burada sona erer.
Ayrıca, Rust sadece farklı olmak için farklı değildir; bir amaca sahip olmak için farklıdır.
Rust'un borç kontrolcüsü (borrow checker), bu farklılık amacı olan benzersiz bir örnektir.
Borrow checker, Rust içinde benzersiz bir özellik olup, tek sahiplik ilkesine ilişkin kuralları zorlayarak güvenli kodlama uygulamalarını teşvik eder.
Başka hiçbir dilde bu özellik bulunmamaktadır. Bu nedenle, borrow checker birçok geliştirici için yabancı bir kavram olabilir, ancak yine de paha biçilmezdir.

Borrow Checker'a biraz daha bakacak olursak, Rust'un bellek güvenliğini sağlamak için geliştirilmiş bir mekanizmadır.
Bu mekanizma, programcıların bellek güvenliği hatalarını önlemesine yardımcı olur.

Borrow Checker, Rust'un sahiplik ve ödünç alma (borrowing) sistemine dayanır.
Temel fikir, bir değerin bellekteki tek sahibinin olduğu ve diğer kod bloklarının bu değeri ödünç alabileceği ancak
sahibinin onu değiştirene kadar sahibinin kilitli olduğu "ölçülebilir" bir sistemdir.
Bu, bellek güvenliği hatalarını önlemeye ve yarışma koşullarını (race conditions) ortadan kaldırmaya yardımcı olur.

Örneğin, Rust'ta bir değişkenin değerini birden fazla kod bloğunda kullanmak istediğinizde, bu değişkeni ödünç alırsınız (borrow).
Bu ödünç alma işlemi sırasında, değişkenin asıl sahibi (ve değişkenin türüne bağlı olarak, değişkenin taşınabilirliği) belirli kısıtlamalara tabidir.

Rust birçok açıdan, öğrenilen derslerin temsilcisidir. Rust'taki bazı benzersiz özellikler, diğer dillerdeki başarısızlık örneklerinden öğrenilen derslerdir.
Rust'un gerekli olduğunda normalden ayrılma isteği, dilin önemli bir özelliğidir.
Birçok programlama dili etkili bellek yönetimi konusunda zorluklar yaşar.

# Fonksiyonel Programlama

Rust, fonksiyonel programlama, ifade odaklı programlama ve desen odaklı programlama gibi çeşitli programlama paradigmalarını benimser.

Fonksiyonel programlama nedir? Bu, bir programlama modelidir ki burada fonksiyonlar dilin temel yapı taşlarıdır.
Fonksiyonel programlamada fonksiyonlar birinci sınıf vatandaşlardır.
Fonksiyonları normalde bir değişkenin bulunduğu her yerde kullanabilirsiniz: yerel bir değişken, fonksiyon parametresi veya bir fonksiyon dönüş olarak.
Bir fonksiyon, diğer fonksiyonlarda işlem bile gerçekleştirebilir, bu durum daha yüksek düzeyde bir fonksiyon olarak tanımlanır.

Rust, fonksiyonel programlamayı hafif bir şekilde benimser.
Dil, tembel değerlendirme, deklaratif programlama tarzı, kuyruk çağrı optimizasyonu gibi fonksiyonel programlamanın her özelliğini içermez.
Ancak, Rust fonksiyonel bir programlama tarzını destekler.

Fonksiyonel programlama dilleri genellikle prosedürel programlama yeteneklerini kısıtlar, örneğin global fonksiyonlar gibi.
Bunlar birbirine ters olmadığı için, Rust prosedürel ve fonksiyonel tarzda programlamayı bir araya getirmeye izin verir.

Saf fonksiyonlar, fonksiyonel programlamanın merkezindedir.
Bir saf fonksiyon olarak, bir fonksiyon, tamamen fonksiyon arayüzü aracılığıyla tanımlanır.
Fonksiyon parametreleri ile belirli bir dönüş değeri arasında doğrudan bir ilişki vardır, yan etkiler olmadan. 
Ek olarak, bir saf fonksiyonun sonuçları tekrarlanabilir olmalıdır.
Örneğin, sonuçları öngörülemez hale getiren içsel bir rastgele sayıya dayanan bir fonksiyon, saf bir fonksiyon değildir.

Değişmezlik, fonksiyonel programlamanın önemli bir öğesi olup, Rust'ın temel ilkelerinden biridir. Saf fonksiyonlar örneğin yan etkileri ortadan kaldırmak için değişmez duruma ağırlık verir. İşaretçiler, global değişkenler ve referanslar genellikle bir saf fonksiyondan kaçınılarak, bir fonksiyondan sızabilecek yan etkilerin önlenmesi amaçlanır.

Özetlemek gerekirse, fonksiyonel programlama birkaç fayda sunar:
- Fonksiyonların birinci sınıf vatandaşlar olarak kullanılmasıyla ek esneklik
- Daha fazla şeffaflık, odak noktası fonksiyonlardır ve bireysel kod satırları değil
- Yan etkilerin fonksiyonlar içindeki yaygın sorunları ortadan kaldıran değişmezlik

- 
