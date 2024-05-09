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
