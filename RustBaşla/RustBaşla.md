# Rust İndirme

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Diğer indirme metodları -> https://forge.rust-lang.org/infra/other-installation-methods.html

# Cargo İndirme

Cargo, Rust programlama dilinin resmi paket yöneticisi ve proje yönetim aracıdır. 

Linux için Cargo'nun İndirilmesi ve Kurulması:
1. Terminali açın ve aşağıdaki komutu kullanarak Rust Programlama Dilini yükleyin:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Rust yükleyici (rustup) yüklendikten sonra, aşağıdaki komutu kullanarak Cargo'yu yükleyin:
```shell
rustup component add cargo
```
MacOS için Cargo'nun İndirilmesi ve Kurulması:

1. Terminali açın ve aşağıdaki komutu kullanarak Homebrew paket yöneticisini yükleyin (eğer yüklü değilse):
```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

2. Homebrew yüklendikten sonra, aşağıdaki komutu kullanarak Rust'u yükleyin:
```shell
brew install rust
```

3. Rust yüklendikten sonra, Cargo otomatik olarak yüklenecektir ve hazır hale gelecektir.

Windows için Cargo'nun İndirilmesi ve Kurulması:
1. Tarayıcınızı kullanarak https://www.rust-lang.org/tools/install adresine gidin.
2. Sayfanın alt kısmında Windows bölümünde bulunan "rustup-init.exe" dosyasını indirin ve çalıştırın.
3. Yükleme sırasında, Rust programlama dilini ve Cargo aracını yüklemek istediğinizi belirten adımları takip edin.

# “Hello, World”

2_1/ "Merhaba, Dünya" uygulamasını göstermektedir. Aktarım kontrol ifadesi olmadan çalışır sırayla, "Hello, World!" metnini görüntüler ve ardından çıkar.

Şimdi programı inceleyelim.

Öncelikle çalıştırılabilir sandık için kaynak kodu .rs uzantılı bir dosyaya kaydedilir, main.rs gibi.
Bu, Rust kaynak dosyaları için standart uzantıdır.
Rust'ta fonksiyonların önünde fn anahtar sözcüğü bulunur.
İşlev adı, parametreler ve varsa dönüş değeri takip eder. 

İşte bir fonksiyonun sözdizimi:

```rust
fn func_name(parameters)->returnval
```

Snake case, Rust'ta fonksiyonlar için kullanılan adlandırma kurallarıdır.
Bu kural için, her kelime fonksiyon adının küçük harfle başlaması ve her bir kelimenin
işlev adının her bir kelimesi küçük harfle başlar ve her bir kelime alt çizgi ile ayrılır.

main fonksiyonu, bir Rust çalıştırılabilir sandığı için giriş noktası fonksiyonudur ve uygulamanın başlangıç noktasıdır.
Rust'ta ana fonksiyonumuzun fonksiyon parametreleri veya açık dönüş değeri yoktur.
Bir fonksiyonun kodu, bir fonksiyonu sınırlayan küme parantezleri {} içinde kapsüllenir.
Ana fonksiyon için, program ana fonksiyon bloğunun sonunda sona erer. Uygulama için birincil iş parçacığıdır.
Ana işlevde, println! macrosu "Hello, World!" mesajını ekrana yazdırır.
Fonksiyon blokları ifadeler, statements ve macro içerebilir. 
Rust'ta macroların adından sonra ünlem işareti (!) bulunur. 
Çoğu durumda, ifadeler ve deyimler noktalı virgül ile sonlandırılır.

# Derle ve Çalıştır

Rust, bir programın gerçek bir binary dosyasına derlendiği, yüksek performanslı bir programlama dilidir. 
Rust, yürütme için sanal bir makine gerektirmeyen natively compiled (doğrudan derlenen) bir dildir.
Bu da demek oluyor ki, Rust projeleri derlendikten sonra bağımsız çalışabilen binary dosyalar haline gelir ve Rust'ın kurulu olmadığı herhangi bir yerde çalıştırılabilir.

Rust'un derleyicisi rustc, Rust projelerini derlemek için kullanılır ve Rust'ta yazılan programların natively compiled (doğrudan derlenen) bir biçimde yürütülmesini sağlar. 
rustc, farklı platformlara uygun olarak farklı türde binary dosyalar oluşturabilir. 
Örneğin, Linux için oluşturulan binary dosyalar genellikle ELF (Executable and Linkable Format) dosyası şeklinde oluşturulurken, 
Windows platformu için taşınabilir yürütülebilir dosyalar (PE - Portable Executable) oluşturabilir.

Bu sayede Rust, hızlı, güvenli ve platformlar arası destek sağlayan bir programlama dilidir ve projelerin bağımsız bir şekilde çalışmasını mümkün kılar.

Burada, rustc aracı "Hello, World" crate'ini çalıştırılabilir bir binary dosyaya derler:

```shell
rustc main.rs
```

Yürütülebilir crate derlendiğinde iki dosya oluşturulur:

- cratename.exe: Bu çalıştırılabilir binary dosyadır.
- cratename.pdb: Bu PDB (program veritabanı), sembolik adlar ve kaynak satırı bilgileri gibi binaryle ilgili meta verileri içerir.
GDB gibi hata ayıklayıcılar, geliştiricilere kullanıcı dostu tanılama bilgileri sunmak için bu PDB dosyasını okur.

rustc, main.rs kaynak dosyasından main.exe ve main.pdb dosyalarını oluşturur.
rustc derleyicisi konuşkandır, ayrıntılı uyarılar ve hata mesajları sağlar. 
Daha fazla ayrıntı için referanslar da sağlanabilir. 

İşte println! makrosu zorunlu ünlem işareti olmadan yanlış kullanıldığında görüntülenen derleyici hata mesajı.

```
error[E0423]: expected function, found macro `print`
--> main.rs:2:2
|
2 | println("Hello, world!");
| ^^^^^ not a function
|
help: use `!` to invoke the macro
|
2 | print!("Hello, world!");
| +
error: aborting due to previous error
For more information about this error, try `rustc --explain E0423`.
```

# Cargo

Rust crate'lerini derlemek ve binary dosyalar oluşturmak için doğrudan rustc derleyicisini kullanmak yerine cargo aracını kullanabilirsiniz. 
Cargo, farklı görevleri yerine getirebilen çok yönlü bir araçtır. 
Bu görevler arasında paket oluşturma ve yönetme, binary dosyaları derleme, güvenli bir ortam sağlama ve bağımlılıkları yönetme bulunur. 
Derleme işlemi için cargo, rustc derleyicisine işleri devreder.

Bu aracın esnekliği sayesinde, Rust kullanıcıları genellikle derleme için rustc yerine cargo'yu tercih ederler. 
Ayrıca, bu durum sadece tek bir komut satırı arayüzü öğrenme anlamına gelir, iki farklı araç yerine. 
Çoğu Rust kullanıcısı zaten cargo'yu paket oluşturmak gibi bir şeyler için kullanıyordur ve 
farklı bir araca geçiş yapmak yerine aynı aracı kullanmak daha basittir.
Ancak unutmayın ki, rustc aracı dolaylı yoldan hâlâ kullanılmaya devam edecektir.

Örneğin, ```cargo new``` komutu, ya yürütülebilir ya da kütüphane tipinde bir crate için yeni bir paket oluşturur. 
Varsayılan olarak yürütülebilir bir crate oluşturulur. 
Kütüphane tipinde bir crate oluşturmak için ise ```--lib``` seçeneği kullanılır.

```
cargo new name
```

cargo new komutu aynı zamanda yeni paket için bir dizin yapısı da oluşturur. 
Başlangıçta bu, bir ana dizin ve src alt dizinini içerir. İhtiyaç duyulduğunda araç daha fazla dizin ekleyecektir.
Ana dizinde .gitignore ve cargo.toml adlı iki dosya bulunur.  Bu, dizin yapısının temelini oluşturur. 
src alt dizininde ise, yürütülebilir veya kütüphane paketi olmasına bağlı olarak main.rs veya lib.rs crate dosyası yer alır.

```
cargo init
```

.gitignore dosyası, GitHub'tan hariç tutulan dizinleri ve dosyaları listeler. 
Başlangıçta, derlenmiş binary dosyaları içeren hedef alt dizini ve cargo.lock dosyası bu listede bulunur.
Cargo.toml, paket için manifest ve yapılandırma dosyasıdır. TOML eki, Tom’s Obvious Minimal Language'e bir göndermedir ve okunabilir bir yapılandırma dosyası için standart bir formattır. 
Cargo.toml, paketle ilgili önemli yapılandırma detaylarını içerir, paketin adı da dahil olmak üzere. 
cargo new komutu, gösterildiği gibi başlangıç cargo.toml dosyasını oluşturur 2_2/ dosyasını inceleyin.

cargo.toml dosyasında aşağıdaki bilgiler sunulmaktadır:
- name: Paket adı, cargo new komutundan türetilir.
- version: Üç parçalı anlamsal sürüm (major.minor.patch).
- edition: Mevcut Rust dil sürümü.
- dependency: Bağımlılıklar bu bölümde belgelenir.
- comments: # karakteri bir yoruma işaret eder, satırın sonuna kadar devam eder.

cargo new komutu cargo.toml dosyasını oluşturur. Lisans bilgileri, kısa bir açıklama ve belgelerin konumu gibi bilgiler el ile TOML dosyasına eklenebilir.
Cargo.toml dosyasının yanı sıra, cargo new komutu src alt dizininde bir kaynak dosyası oluşturur. 
Yürütülebilir bir kasa için, bu "Hello, World" uygulaması için örnek kod içeren main.rs dosyasıdır.
Tabii ki, bunu gerçek kodunuzla değiştirmekte özgürsünüz.

Aşağıdaki gibi bir komutla kasayı derleyebilir ve binary yürütülebilir dosyayı oluşturabilirsiniz:

```
cargo build
```

Bu komut paket içinden çalıştırılmalıdır. Cargo build komutu artımlı bir derleme yapar.
Kasaya yapılan değişiklikler, cargo.toml'deki bağımlılıkların değiştirilmesi ve diğer nedenler artımlı bir derleme yerine tam bir derlemeyi gerektirebilir.

Cargo build komutu bir target/ dizini oluşturur. Bu dizinin içinde, binary hedefin türüne bağlı olarak bir debug veya release dizini oluşturulur. 
Cargo build komutu varsayılan olarak bir debug hedefi oluşturur. 
Bir debug ikilisi, hata ayıklamak için ideal olan, hatalı yatırımlar yapmayan optimizasyonlara sahiptir. 
Release ikilisi ise çoğunlukla performans veya boyut için optimize edilir.
```cargo build --release``` komutu, release dizinine yerleştirilen bir release ikilisi oluşturur.

Özet geçecek olursak,

cargo build komutu, Rust projesini derlemek için kullanılır.
Bu komut, proje dosyalarını derleyerek ikili dosyalar oluşturur ve çalıştırılabilir bir program oluşturur.
Bununla birlikte, --release bayrağı olmadan kullanıldığında, derleme işlemi daha hızlı tamamlanır ancak oluşturulan ikili dosyalar optimize edilmemiş olur. 
Bu nedenle, --release bayrağı olmadan kullanıldığında, derlenen kod hızlıca test etmek veya hata ayıklamak için kullanılabilirken,
optimize edilmiş bir sürüm elde etmek amacıyla --release bayrağı kullanmak daha mantıklı olabilir.

cargo build --release komutu, Rust projesinin optimize edilmiş bir sürümünün derlenmesini sağlar. 
Bu komut, üretim ortamlarında kullanılmak üzere en iyi performansı elde etmek için kodu optimize eder. 
Derlenen ikili dosya, release modunda bulunan release dizinine yerleştirilir.

--release bayrağı, derlenen ikili dosuyun en iyi performansı sağlamak için optimize edilmesini sağlar. 
Bu optimizasyonlar genellikle kodun daha hızlı çalışmasını ve daha küçük boyutlara sahip olmasını sağlar. 
Bu nedenle, --release bayrağı ikili dosyanın daha optimize edilmiş bir sürümünü oluşturur.

Eğer bir executable crate'inizi çalıştırmak istiyorsanız, ``cargo run`` komutunu kullanabilirsiniz. 
Bu komutun da paketin içinden çalıştırılması gerekmektedir. Eğer daha önce ikili dosya oluşturulmadıysa, önce cargo build komutu otomatik olarak çalıştırılacaktır. 
Bu nedenle, bazıları executable crate'ler için ayrı bir build adımını tamamen atlayarak sadece cargo run komutuna güvenirler.
Bu sayede ikili dosyayı oluşturup çalıştırmak için ayrı ayrı adımlarla uğraşmaya gerek kalmaz.

# Kütüphane(Library)

Cargo kullanarak bir kütüphane oluşturduğunuzda, temel fark "main.rs" yerine "lib.rs" dosyasının oluşturulmasıdır ve 
"Hello, World" uygulaması yerine basit bir matematiksel işlemi gerçekleştiren bir fonksiyon içeren örnek kod bulunur. 
Tüm bunlar bir birim testi içinde gerçekleşir. Aşağıdaki komut yeni bir paket oluşturur ve içinde bir kütüphane oluşturur:

```
cargo new --lib paketismi
```

Yürütülebilir binary dosyaların aksine, kütüphaneler kendiliğinden çalışmazlar. 
Kütüphane için kaynak kodunu bir birim testi içine yerleştirmek, o kodu yürütmek için bir mekanizma sağlar. 

Kütüphane crate aşağıdaki komutla çalıştırabilirsiniz:

```
cargo test
```

Bu komut, kütüktecratede bir birim testleri çalıştıracaktır. 
lib.rs için, bu kütüphanedeki kaynak kodunu yürüterek test etme olanağı sağlar. 
Sonuç olarak, cargo test komutu birim testinin başarılı olup olmadığını gösterir.
2_3/, cargo'nun oluşturduğu lib.rs'yi örnek kod içererek gösterir.

lib.rs kütüphanesini inceleyelim. Dosya #cfg(test) açıklaması ile başlar. 
Bu açıklama, cargo build komutuna birim testlerini ihmal etmesini ve sonuç ikili dosyasına dahil etmemesini istemektedir.
Dosya içinde her bir birim testi #test açıklaması ile etiketlenir. 
Örnek bir birim testi basit bir toplama işlemi gerçekleştirir. İşlem sonucu, asserteq! makrosu içinde beklenen değerle karşılaştırılır. 
Kodu birim testlerde, kütüphanenizin belirli genel fonksiyonlarına referans vermek için güncellemelisiniz. 
Kütüphanedeki her genel fonksiyon için bir birim test olmalıdır.

# Yorumlar

Bu bölümdeki kaynak kodlarına şimdiye kadar yorum eklenmemiştir. 
Yorumlar, yazarı belirleme, lisans bilgilerini vurgulama, işlevleri belgeleme, karmaşık algoritmaları açıklama veya 
sadece önemli bağlam sağlama gibi çeşitli nedenlerle kaynak kodlarında yer alır.

Rust, C tarzı yorumları destekler.
// karakterleri bir yorumu başlatır ve satırın sonuna kadar devam ettirir. 
Çok satırlı yorumları / ve / karakterleri arasında çerçeveleyebilirsiniz.

2_4/ dosyada, “Merhaba, Dünya” kaynak koduna yazarı, 
kaynak dosyasını ve uygulamanın amacını tanımlayan yorumlar eklenmiştir. 
Hem çok satırlı hem de tek satırlı yorumlar gösterilmiştir.

Rust ayrıca belgelendirme yorumlarını destekler. Belgelendirme yorumları rustdoc aracını kullanarak HTML sayfasına derlenir. 
Bu araç otomatik olarak Rust araç zincirine dahildir. Belgelendirme yorumları derlendiğinde, {package} / hedef / doc / {package} dizininde birkaç HTML dosyası oluşturulur. 
Ana HTML dosyası index.html'dir ve herhangi bir tarayıcıda açılabilir.

Belgelendirme için tek satırlı ve çok satırlı yorumlar bulunmaktadır - her ikisi de markdown'u destekler.
Tek satırlı belgelendirme yorumları /// karakterlerini kullanır. 
Çok satırlı belgelendirme yorumları / ve / karakterleri arasına alınır. 
Belgelendirme yorumları kaynak dosyasındaki bir sonraki varlık için geçerlidir, örneğin bir struct veya bir fonksiyon için.
Şimdi başka bir "Merhaba, Dünya" uygulamasının sürümüne bakalım - bir kütüphane paketi olarak. 
Bu kütüphane, çeşitli dillerde bir selam döndüren hello_world fonksiyonunu ortaya çıkarır. 
Fonksiyon, "Merhaba, Dünya!" mesajını göstermek için bir ana-minor dil kodunu kullanarak belirli bir dil seçen bir parametreyi kabul eder. 
2_5/ dosyası, belgelendirme yorumlarıyla fonksiyonu göstermektedir.

Bu kod, verilen dil koduna (`language`), karşılık gelen bir "hello world" metnini döndüren bir işlev tanımlar.
İşlev, `hello_world` olarak adlandırılmıştır ve bir dil kodu alır (`&str` türünde) ve buna göre uygun bir metin döndürür (`&'static str` türünde).

İşlevin içeriği şu adımları izler:

1. `match` ifadesi, `language` değişkeninin değerine göre karşılaştırma yapar.
2. Her dil kodu için bir durum (case) belirtilir.
3. İfade, dil koduna (`language`) bağlı olarak farklı bir metin döndürür.
4. Eğer belirtilen dil kodu (`language`) desteklenmiyorsa, varsayılan olarak "Hello, world!" metni döndürülür.

//! <b>Hello kütüphane paketi</b>
//! <p>Yazar: Donis Marshall</p>
//! <p>Apache 2.0 Lisansı</p>

Belgelendirme yorumları, genellikle mevcut kasanın yerine geçen üst öğeye uygulanan yorumlardır.
Aşağıda, işaretleme dahil olmak üzere hello kütüphane paketi için belgelendirme örneği bulunmaktadır:

Belgelere örnek kod eklemek de mümkündür. Bu, uygulama kullanıcılarına çok d
eğerli bir rehberlik sağlayabilir. 
Belgelerdeki örnek kodlar ayrıca birim testleri olarak da çalıştırılabilir.
Örnek kodu Belgeler bölümüne yerleştirin.
Bölümler /// # karakterleri ile belirtilir. /// ve # arasında bir boşluk olduğuna dikkat edin. 
Kod parçacığını üç ters tırnak (///```) ile baştan ve sondan çerçeveleyin. 2_6/ örnek kodu göstermektedir.
