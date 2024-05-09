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

Rust, çeşitli programlama paradigmalarını benimser.
Bu paradigmalar arasında fonksiyonel programlama, ifade odaklı programlama ve desen odaklı programlama bulunur.

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

Değişmezlik, fonksiyonel programlamanın önemli bir öğesi olup, Rust'ın temel ilkelerinden biridir.
Saf fonksiyonlar örneğin yan etkileri ortadan kaldırmak için değişmez duruma ağırlık verir. 
İşaretçiler, global değişkenler ve referanslar genellikle bir saf fonksiyondan kaçınılarak, bir fonksiyondan sızabilecek yan etkilerin önlenmesi amaçlanır.

Özetlemek gerekirse, fonksiyonel programlama birkaç fayda sunar:

- Fonksiyonların birinci sınıf vatandaşlar olarak kullanılmasıyla ek esneklik
- Daha fazla şeffaflık, odak noktası fonksiyonlardır ve bireysel kod satırları değil
- Yan etkilerin fonksiyonlar içindeki yaygın sorunları ortadan kaldıran değişmezlik

# İfade Odaklı

Rust aynı zamanda ifadelerle odaklı bir dil olan, çoğu işlemin bir değer döndürdüğü,
bir şey döndürmeyen ifadelerin aksine ifadelerin çoğu bir değer döndürdüğü bir programlama tarzıdır.
İfadelerle odaklı programlama, fonksiyonel programlamanın yakın akrabasıdır.
Tüm fonksiyonel programlama dilleri aynı zamanda ifadelerle odaklı dillerdir.

Peki bir ifade nedir?

İfadeler bir değer döndürmez ancak yan etkiye sebep olabilir.
Mümkün yan etkiler, veritabanı manipülasyonu veya paylaşılan bir değişkenin güncellenmesi gibi sınırsız olabilir.
Gerçekten, bazı ifadelerin amacı yan etkidir.

İfadeler bir veya daha fazla işlemi içeren ve bir değer döndüren, minimal veya hiç yan etkisi olmayan ifadelerdir.
Saf fonksiyonlar ifadelerin örnekleridir.

Rust'ta ifadeler tercih edilir, hatta if ve while gibi kontrol transfer ifadeleri aslında ifadelerdir.

İfadelerle odaklı programlamanın birçok avantajı şunlardır:
- Yan etkiler olmadan ifadelerle odaklı programlar daha kolay bakım yapılabilir.
- Bir ifadenin değeri tamamen arayüzüyle tanımlanır. Bu, ifadelerin daha şeffaf olmasını sağlar.
- İfadeler arayüz yönlendirildiği için, daha test edilebilirler.
- İfade odaklı programlama, belgelendirmeyi kolaylaştırır. Yan etkiler olmadan, ifade belgelendirme olarak da kullanılabilir.
- İfadeler daha kolay bir şekilde birleştirilebilir.

1_1/ de, Rust'ta hem fonksiyonel hem de ifadelerle odaklı programlamanın bir örneği. 

# Desen Odaklı

Rust programlamada, desenlerin sıkça kullanıldığını söylemek doğru olacaktır.
Desenler, Rust kodlamasında yaygın olarak bulunur.
Bu, desen eşleme pratiklerine olan vurgu, Rust programlama tarzının benzersizliğine katkıda bulunur.
Örneğin, Rust kaynak kodu, C++ veya Java'dan farklı bir görünüme sahiptir!

Desen eşleme genellikle switch ifadeleri ile ilişkilendirilir.
Örneğin, C++, Java, Go ve diğer dillerde switch ifadesi bulunmaktadır.
Bu, genellikle string veya integral ifadelerine dayanan tek boyutlu desen eşleme anlamına gelir.
Rust'ta ise desen eşleme, kullanıcı tanımlı tiplerin ve dizilerin örneklerine genişletilmiştir.

Rust'un bir switch ifadesi yerine bir eşleme ifadesi olan match ifadesi bulunmaktadır.
Ancak, desen odaklı programlama match ifadesinin ötesine uzanır.
Rust'ta, herhangi bir ifadenin her örneği, desen eşleme için bir fırsattır.
Örneğin, hatta basit bir atanma veya koşullu ifade bile desen eşleme ile sonuçlanabilir.
Bu, kodunuzu yeniden hayal etmenin ilginç yollarını sunar.

Rust'ta desen odaklı programlamanın birkaç faydası bulunmaktadır:
- Rust'taki desenler oldukça ifadecidir. Bu, karmaşık kodları daha basit ifadelerle birleştirme olanağı sağlar.
- Rust'ta desen odaklı programlama, ifade odaklı programlamaya tamamlayıcı bir model sunar.
- Rust, kapsamlı desen eşlemeyi destekler, bu da daha güvenilir ve hata yapmaya daha az yatkındır.

 1_2/ de gösterildiği gibi desen eşleme örneği.

# Özellikler

Rust'a ve popülerliğine katkıda bulunan temel özellikleri inceleyelim.

# Güvenlik

Güvenlik, dili dokunan hemen hemen tüm yönleri etkileyen önemli bir özelliğe sahiptir. 
Güvenli kod, sağlam, öngörülebilir ve beklenmedik hatalara eğilimli değildir. 
Bu özelliklerle, Rust güvenle uygulama geliştirebileceğiniz sağlam bir temel sunar.
Değiştirilemez değişkenler, tek sahiplik ilkesi ve diğer özellikler, bu hedefe katkıda bulunur.
Ayrıca, Rust, derleme zamanında güvenli kod yazma uygulamalarını zorunlu kılar.
Ödünç alma denetleyicisi ile sahiplik modeli bunun mükemmel bir örneğidir. 
Derleme zamanında, ödünç alma denetleyicisi, sahiplik de dahil olmak üzere bir dizi denetleme yapar. 
Eğer sahiplik kontrolü başarısız olursa, ödünç alma denetleyici bir açıklama sunar ve derleme başarıyla tamamlanmaz.

Rust'ta güvenli koda katkıda bulunan birkaç faktör var:
- Değiştirilemezlilik varsayılan olarak yapılır, bu da kazara değişiklikleri engeller.
- Dangling referanslar gibi anti-örüntülerin önlenmesi için uygun ömürlerin zorunlu kılınması
- Güvenli işaretçiler için referanslar
- Değişebilen boyutlarda kaynaklar için güvenilir bellek yönetimi için "bir kaynağı edinme işlemi başlatmak" (RAII) stratejisi, örneğin vektörler gibi.

# Sahiplik

Sahiplik özelliği, tek sahip ilkesini kullanarak güvenli bellek erişimi sağlar.
Bu ilke, değişkenlere tek bir sahip atar ve asla birden fazla sahip olmaz.
Bu yaklaşım, aynı belleğin sahipliğinin paylaşılmasını önler. 
Bu yaklaşımla önlenebilecek potansiyel sorunlar arasında yarış koşulları, kararsız değişkenler ve dangle referanslar yer alır.

Tek sahip ilkesini göstermek için bir araba benzetmesi kullanalım. İşte temel gerçekler:

Bir araba var ve Bob onun tek sahibi.

Şimdi iki senaryo var:
- Bob'un arabası var.
- Ari zaman zaman aynı arabayı kullanmak istiyor.

Sahip olarak, Bob her zaman arabayı kullanabilir, başka birine ödünç vermedikçe. 
Başka biri (Ari) arabayı kullanmak istediğinde iki olasılık vardır:

Bob, aracı ya satmak ya da Ari'ye ödünç vermek zorundadır. 
Her durumda, Bob en azından geçici olarak aracın sahipliğini kaybeder.

Bob aracı Ari'ye ödünç verirse adımlar şöyle olur:

- Bob aracı kullanır.
- Bob aracı Ari'ye ödünç verir. Ari aracı kullanır. Ari bitirdikten sonra aracı Bob'a geri verir.
- Bob aracı kullanır.

Ödünç alma denetleyicisi, derleme zamanında doğru sahipliği, ödünç verme dahil, zorunlu kılmaktan sorumludur. 

# Ömürler(Lifetimes)

Ömürler, Rust'ta artık mevcut olmayan değerlere erişimi engelleyen bir özelliktir.  Bir referans, Rust'ta temel bir işarecidir. 
İzin verilirse, bırakılan değerlere yanlış erişim, askıda kalan referanslar ve potansiyel program hatası oluşturabilir. 
Sonuç, istikrarsız ve öngörülemeyen uygulamalar olacaktır. Rust, bu sorunu ömürler modeli ile ortadan kaldırır.

Ödünç alma denetleyicisi ömürleri yönetir. Geçersiz ömürler hakkında derleme zamanında bilgilendirilirsiniz.
Bu tür sorunlar derleme zamanında daha iyi izole edilir, çalışma zamanında değil.

Ömürlerle ilgili belirsizlik olduğunda, ömür işaretleri, ödünç alma denetleyicisine doğru ömür hakkında ipuçları verir. 
Ömür belirlemenin basit olduğu durumlarda ömür işaretleri gerekli değildir. Ödünç alma denetleyicisi sadece bilecektir. Buna ömür çarpıklığı denir.
Ömürler özelliğinin faydası, askıda kalan referans endişesi olmadan kararlı bir bellek ortamıdır.

# Korkusuz Eşzamanlılık

Korkusuz eşzamanlılık, önemlidir ve Rust'ın temel özellikler listesine dahil edilmesi gereken bir özelliktir.
Korkusuz eşzamanlılık, eşzamanlı programlama için güvenli bir ortam sağlar. 
Bu güvenli ortam, önce bahsedilen özelliklerin faydasıyla oluşturulur. 
Örneğin, Rust'taki sahiplik modeli, genellikle eşzamanlı programlamadaki yarış koşullarını ortadan kaldırır.

Dizisel programlamadan eşzamanlı programlamaya geçerken genellikle çok iş parçacıklı kodlar için güvenli bir ortamı sağlamak için bir sertifikasyon süreci yapılır. 
Genel değişkenleri, paylaşılan veriler olarak kaldırmak, tipik olarak sertifikasyon sürecinde atılan adımlardan biridir.
Korkusuz eşzamanlılık, sertifikasyona ihtiyacı ortadan kaldırır.

# Maliyetsiz Soyutlama

Maliyetsiz soyutlama, Rust'ın özelliklerinden biridir. Evet, doğru okudunuz. 
Bu nedenle, burada bahsedilen son özelliktir. Maliyetsiz soyutlama, Rust özelliklerinin, mümkünse çalışma zamanında performans cezası almadığı ilkesidir.
Jenerasyonel çöp toplama, Java, C# ve Go gibi birkaç popüler yönetilen dilin içsel bir özelliğidir ve dinamik belleği yönetmek için kullanılır. 
Çöp toplama maliyetli ve belirlenemeyen bir işlemdir. Bu nedenle, çöp toplamanın ne zaman gerçekleşeceğini asla bilemezsiniz. 
Buna Rust'ı karşılaştırın, burada hiçbir bellek modeli yok. 
Gerçekten hiç yok! Önceki bölümde açıklandığı gibi sahiplik, ek maliyet olmadan belirlenebilir bellek yönetimi sağlar.
Bu, maliyetsiz soyutlamanın bir örneğidir.

# Rust Metodolojisi

