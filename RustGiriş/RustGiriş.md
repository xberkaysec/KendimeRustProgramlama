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

Birçok teknoloji, programlama dilleri de dahil olmak üzere, kendi terminolojilerine sahiptir. 
Bu terminolojiyi öğrenmek, akranlarınızla ve geniş Rust topluluğundaki diğer kişilerle iletişim kurmanıza yardımcı olabilir.

Rust için motif, taşınma kasaları olan "crates"tir.

Rust: En önemli terimle başlayalım: "Rust" kendisi. Rust, bir akronim veya özel bir teknoloji terimi değildir.
Rust adı, yaşayan bitkilere saldıran güçlü bir patojen olan pas mantarından gelmektedir.

Orijinal Rust tasarımcısı Graydon Hoare, şu açıklamayı yapmıştır: "Ona fungi adını verdim. Paslar inanılmaz yaratıklardır."

Crate: Bir crate, Rust'ta bir derleme birimidir. 
Yürütülebilir, kütüphane veya harici crates en yaygın olanlardır.

Yürütülebilir crate: Bir yürütülebilir crate, diğer cratelardan bağımsız olarak başlatılabilen bir ikili yürütülebilir dosyadır.

Kütüphane crate: Kütüphane crateleri, diğer cratelere hizmetler sağlar ve bağımsız olarak yürütülmez.

Harici crate: Harici crateler, harici bağımlılıklardır. Örneğin, Crate A, Crate B'yi referans alır, ancak Crate B aynı pakette değildir. 
Bu nedenle, Crate B, Crate A için bir harici crate veya bağımlılıktır.

Paketler: Bir paket, belirli bir hizmet sağlayan birden fazla crate içerir. 
Paketler, birden fazla yürütülebilir crate ve olası bir kütüphane crate içerebilir.

Modüller: Rust'taki modüller, diğer programlama dillerinden gelen isim alanlarına benzerdir. 
Modülleri kullanarak bir crate içinde hiyerarşik bir program yapısı oluşturabilirsiniz.
Modüller ayrıca isim çakışmalarını önlemeye yardımcı olur.

Cargo: Rust'ta birkaç cargo varlığı vardır, bu da crates motifini genişletir (cargo kasaları).

Cargo aracı: Cargo aracı, Rust paket yöneticisidir.

Cargo.toml: Cargo.toml dosyası, Rust için manifest ve yapılandırma dosyasıdır.

Cargo.lock: Cargo.lock dosyası, tüm bağımlılıkların belirli sürümleri ile bir kaydı tutar.

RS: RS (Rust source), Rust kaynak dosyaları için uzantıdır.

Rust terminolojisindeki kavramların gerçek projelerde nasıl kullanılabileceğini hakkında,
Örneğin, bir Rust projesi oluşturduğunuzu ve bu projede bazı kütüphanelere ihtiyacınız olduğunu varsayalım. 
Bu durumda, projenizin ana klasöründe Cargo.toml dosyasını oluşturarak bu kütüphaneleri belirtmeniz gerekecektir.
Bu dosyada, projenizin bağımlılıklarını ve diğer yapılandırma ayarlarını tanımlayabilirsiniz.

Aynı projede, farklı işlevleri yerine getiren modüller oluşturabilirsiniz. 
Örneğin, bir matematik işlevlerini içeren bir modül ve bir dosya işlemlerini içeren başka bir modül oluşturabilirsiniz. 
Bu modüller sayesinde, kodunuzu daha organize bir şekilde tutabilir ve daha iyi okunabilir hale getirebilirsiniz.

Projenizde kullanmak istediğiniz harici crate'ler için de Cargo.toml dosyasına bu crate'leri ekleyebilirsiniz.
Böylece, projenizin dışındaki bu harici bağımlılıkları projenize dahil edebilir ve kullanabilirsiniz.

Son olarak, projenizi derlemek ve yönetmek için Cargo aracını kullanabilirsiniz. 
Cargo, belirtilen bağımlılıkları indirip yükleyerek, projenizi derleyip çalıştırmanıza olanak tanır. 
Cargo.lock dosyası ise tüm bu bağımlılıkların belirli sürümlerinin kaydını tutar ve projenizin bağımlılıklarının güvenli bir şekilde yönetilmesini sağlar. 

# Araçlar(Tools)

Rust ortamı, Rust kaynak kodunu derlemeden, rust toolunu yüklemeye, deposu publize edilene kadar çeşitli hizmetler sunan birçok araca ev sahipliği yapmaktadır. 
Bu araçları anlamak, üretkenliğinizi arttıracaktır. 

- Rustup aracı: Rustup aracı, Rust kurulum aracıdır. Ayrıca araç takımını da kurar.
- Rust yükleyicisini https://rustup.rs adresinden indirebilirsiniz. Rust'u başarılı bir şekilde yüklemek için oradaki talimatları takip edin.

- Cargo aracı: Cargo, paket yöneticisi olarak temel rolü olan çok amaçlı bir araçtır.
- Ek hizmetler arasında kod derleme, kaynak kodu biçimlendirme ve yeni crateler oluşturma bulunmaktadır.

İşte bir kütüphane crate oluşturan bir Cargo ifadesi:

cargo new --lib mylib

- Rustc aracı: Rustc, Rust derleyicisidir. Rustc, bir Rust kaynak dosyasını (.rs) çalıştırılabilir veya kütüphane ikiliye derleyebilir.

İşte basit bir crate oluşturmak için Rustc ifadesi:

$ rustc kaynak.rs

- Rustdoc aracı: Rustdoc aracı, Rust kaynak dosyasına gömülmüş belge yorumlarını HTML'de render edilmiş bir yardım belgesine derler.

- Clippy aracı: Clippy, çeşitli linterlardan oluşan kapsamlı bir test aracıdır.
Bu araç, kodunuza fayda sağlayabilecek yaygın problemleri ve en iyi uygulamaları tanımlar.

- Rustfmt aracı: Rustfmt aracı, Rust için stil kılavuzlarına uygun olarak kaynak dosyalarını yeniden biçimlendirir.

Cargo, Rust ortamında kilit bir araçtır. Çevrenizi ve paketlerinizi korumak için gereken görevlerin birçoğunu yapmak için Cargo'yu kullanabilirsiniz.
