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

