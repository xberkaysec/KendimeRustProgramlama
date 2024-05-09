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

Rust derleyicisinin konuşkanlığına rağmen, derleme sırasında ilgili tüm hata bilgilerini görüntülemek zaman zaman pratik değildir. 
Bu durumda, bir hata tanımlayıcısı sağlanır. 
```rustc --explain erroridentifier```  komutunu kullanarak ek hata bilgilerini görüntüleyebilirsiniz. 
Ek bilgiler, hatanın ayrıntılı bir açıklamasını, sorunun düzeltilmesine ilişkin önerileri, örnek kodu ve daha fazlasını içerir.
