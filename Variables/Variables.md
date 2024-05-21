# Giriş

Rust programlama dilinde, type sistemi mevcut tipleri ve özelliklerini kapsar.
Elbette, Rust'un type sisteminde genel hedefi olan güvenlik, sağlamlık ve ölçeklenebilirlikle ilgili benzersiz özellikleri vardır.
Type sistemi, programlama dilinin her yönünü etkiler. 
Örneğin, değişkenlerin değiştirilebilirliği, dil üzerinde geniş bir etkiye sahiptir. 
Type sistemi, bir uygulamayı bağlayan ipucudur.
Bu nedenle, Rust'un kapsamlı type sisteminin tam anlaşılmasının önemli olduğu açıktır.

Rust, güçlü bir type sistemine sahip bir dildir. 
Bu nedenle, değişkenler statik olarak tipe sahiptir. 
Değişkenin türü, bildirildiği anda belirlenir ve daha sonra değiştirilemez. 
Tür çıkarımı ile bile, bir değişkene atanan bir çıkarılan tür, asla değişmez. 
Bu, değişkenlerin yanlış kullanımından kaynaklanan hataları içeren zayıf type sistemlerine sahip dillerde yaygın olan problemleri önler.

Ayrıca, Rust diğer dillere göre, dolaylı çıkarımlar da dahil olmak üzere, doğru türün çıkarılması konusunda daha esnek bir yapı sunar.
Tür çıkarımı, bir değişkenin veya ifadenin türünün açıkça belirtilmemesi durumunda, derleyicinin kodu inceleyerek doğru türü belirlemesi işlemidir. 
Dolaylı çıkarım ise, tür çıkarımının daha karmaşık veya dolaylı yollarla gerçekleştiği durumları ifade eder.
Rust'ta değişmezlik varsayılan olarak gelir ve dilin güvenliği ve sağlamlığına katkıda bulunur. 
Varsayılan olarak, değişkenler değiştirilemezdir. Bu, istenmeyen değişiklikleri önler ve şeffaflığı arttırır.
Rust'un esnek tür çıkarımı, bu tür belirsizliği durumlarında bile güvenli ve doğru türlerin tespit edilmesine olanak tanır. 
Bu, kodun daha okunabilir olmasını ve aynı zamanda hataları azaltmayı sağlar.

Rust, standart türler ve operatörlerin tam bir setini sağlar. 
Bunun yanı sıra, daha karmaşık problemleri modellemek için özel türler veya toplu türler oluşturabilirsiniz. 
Bir muhasebe sistemi, bir blok zinciri veya hatta uzay mekiği gibi şeyleri bile modelleyebilirsiniz!

Rust tür sisteminin bir diğer faydası da geniş bir tür boyutu çeşitliliğidir. 
Bu, her bitin önemli olduğu uygulamalarda geliştiricilere bellek kullanımını verimli bir şekilde yönetme imkanı sağlar.

# Terminoloji

Bu bölümde sıkça geçen ve ilişkili olan terimler type(tür), variables(değişkenler) ve memory(bellek)'dir. 
Aslında, bu terimler birkaç kez zaten geçti. Bellek, verilerin bulunduğu yerdir. 
Değişkenler ise adlandırılmış bellek depolama alanlarıdır ve verilere bellek adreslerini kullanarak referans verme ihtiyacını ortadan kaldırırlar.
Sembolik isimler çok daha kolay hatırlanır! Belleğin içsel bir formatı yoktur. 
Bir tür, bir integer veya float gibi bir değerin bellek düzenini tanımlar.

Variable Binding(değişken bağlama), Rust type sisteminde başka bir önemli terimdir. 
Bir değişken adıyla bellek arasında bir bağ oluşturan `let` ifadesi gibi bir bildirim, bir değişken adı ile bellek konumları arasında bir bağ oluşturur. 
Başka bir deyişle, bellek konumları değişkenlere bağlanır. Rust esnek bağlama destekler.

Bu terminoloji, Rust dilinde değişkenlerin bellek üzerinde nasıl çalıştığını ve türlerin nasıl tanımlandığını anlamak için önemlidir. 
Bu terimlerin doğru bir şekilde anlaşılması, dilin kullanımını daha verimli ve etkili hale getirebilir.

# Variables

Değişkenler, belirli bir bellek konumunu tanımlayan bir bellek adresine çözülür. 
Değişken adı, ham bir bellek adresinden daha açıklayıcı ve tutarlıdır. 
Bu, kod yazarken değil sadece bir uygulamayı bakım yaparken veya hata ayıklarken son derece değerlidir. 
Kendini belgeleyen kod yazarken, açıklayıcı isimler değişmez değişkenler için esastır.
Değişken adları için kurallar ve adlandırma kuralları şunlardır:

- Büyük-küçük harfe duyarlıdır.
- Alphanumeric karakterler ve alt çizgi içerirler.
- Bir sayı ile başlayamazlar.
- Adlandırma kuralı "snake case" şeklindedir.
  
`let` ifadesi ile statik olmayan bir yerel değişken bildirebilirsiniz. 
Değişkenin türü, let ifadesi içinde açıkça belirtilir veya tür çıkarımı yoluyla belirlenir. 
Her iki durumda da, değişken statik olarak tiplendirilir. Değişkeni let ifadesiyle veya daha sonra başlatılabilir.
Ancak, değişkenler herhangi bir şekilde kullanılmadan önce başlatılmalıdır.

İşte bir 'değişmez' değişken bildirmek için farklı sözdizimleri:

```
let degisken_adi:tur=deger;
let degisken_adi=deger;
let degisken_adi:tur;
let degisken_adi;
```

# Primitives

Rust'un temel tipleri ve yapı taşları olan "primitives"ler, daha karmaşık tipleri oluşturmak için bir araya getirilebilir. 
Bu temel tipler, Rust derleyicisi tarafından özgün olarak uygulanmıştır. 
Primitiflerin işlevleri ve öznitelikleri, genellikle Rust kütüphanelerinde uygulanmıştır.
Örneğin, i32::MAX, i32 primitifine uygulanmış bir ilişkili sabittir.

Skaler primitifler, tek bir değeri temsil eden ve bellekte sabit bir boyutta saklanabilen temel veri tipleridir. 
Bunlar, genellikle bir boyut ve basit bir veri yapısına sahiptir.
Rust'un hem skaler hem de skaler olmayan primitifleri vardır. Skaler primitifler şunları içerir:

- İşaretli tamsayılar
- İşaretsiz tamsayılar
- Ondalık sayılar (float)
- Mantıksal değerler (bool)
- Referanslar

Skaler olmayan primitifler, tek bir değerden daha karmaşık veri yapılarına sahip olan temel veri tipleridir. 
Bunlar, koleksiyonlar ve daha karmaşık veri yapılarını temsil eder. 
Skaler olmayan primitifler şunları içerir:
- Dizi (array)
- Demet (tuple)
- Dilim (slice)
- String
- str

Diğer primitif tipler şunları içerir:
- (): ünite tipi (unit type)
- fn: işaretçi fonksiyon tipleri (function pointer types)
- Ham işaretçi (raw pointer)

Bu bölümde, skaler primitif tipleri gözden geçireceğiz.
Daha sonraki bölümlerde, diziler ve stringler gibi diğer primitifler ele alınacaktır.

# Integer Tipi

isize ve usize hariç olmak üzere, tamsayı türleri sabit boyutlu olup, tür adının sonuna eklenen ek bit boyutunu belirtir. 
Örneğin, i64 işaretli 64 bit tamsayıdır.

Aşağıdaki işaretli tamsayı türleri bulunmaktadır:
- isize
- i8
- i16
- i32
- i64
- i128

Bu da işaretsiz tamsayı türleridir:
- usize
- u8
- u16
- u32
- u64
- u128

isize ve usize türlerinin boyutu, çalışma ortamına bağlıdır. Bu, bir işaretçinin boyutudur. 
Merak ediyorsanız, bir türün boyutunu size_of metodu işlevi ile doğrulayabilirsiniz. 
Bu metod, std::mem modülünde bulunmaktadır.
-> Kod Örneği 3_1. usize boyutunu doğrulama kodu.

Önerildiği durumlarda, işaretli tamsayı türünün varsayılan tipi i32'dir. 
İşaretsiz tamsayılar için ise varsayılan tip u32'dir. 
Öntanımlı bir tamsayı değerinin varsayılan türünü gösteren Kod Örneği 3_2. 
Bu kod, genel kavramlar, Insight(içgörü) ve any türünü içerir. 

Daha iyi okunabilirlik için, sayısal literallerde ve ondalık sayılar arasında alt çizgiler kullanılabilir. 
Genellikle alt çizgiler, 103'lük segmentleri belirtir. Ancak, Rust alt çizgi polisi değildir. 
Alt çizgiyi sayının herhangi bir yerine koyabilirsiniz.

```
let normal1 = 123_456_678;
let normal2 = 123_456.67;
let ilginc = 12_3_456;
```

# Overflow

Overflow taşması veya alt taşması, bir tamsayı türünün minimum veya maksimum değerinin aşılması durumunda meydana gelir. 
Genel olarak, her ikisi de overflow olarak kabul edilir.
Taşma meydana geldiğinde, sonuç debug veya release derlemesi olmasına bağlıdır. 
Eğer bir debug derlemesi ise, taşma olduğunda bir panik oluşacaktır. 
Panikler, işlenmemiş bir uygulamayı durdurabilen istisnai olaylardır. 
Ancak, bir taşma release derlemesinde meydana geldiğinde panik oluşmaz. 
Taşma, sayıyı maksimum değerden minimuma ya da tam tersine kaydırır.
3_3 Kod Listesi'nde gösterildiği gibi, sayı, maksimum i8 değeri 127'den minimum i8 değeri -128'e taşma olduğunda döner.

-> Kod Listesi 3_3 Bir overflow oluşturma.

Bir alt taşma da benzer şekilde, ancak diğer yönde döner, -128'den 127'ye, 
3_4 Kod Listesi'nde gösterildiği gibi.

-> Kod Listesi 3_4 Bir alt taşma oluşturma

Taşmanın, derleme hedefine bağlı olarak tutarsız sonuçları sorunlara yol açabilir. 
overflowing_add fonksiyonu, tutarlı sonuçlar sağlayan bir alternatif sunar. 
Bu fonksiyonla, sonuç derleme hedefine bakılmaksızın aynı olacaktır. 
overflowing_add fonksiyonu, toplama işlemi yapar ve sonucu ve taşma durumunu içeren bir demet olarak döndürür.
Eğer bir taşma olursa, taşma durumu true olarak ayarlanır. 3_5 Kod Listesi, overflowing_add fonksiyonunu kullanmanın bir örneğini sağlar.

-> Kod Listesi 3_5 overflowing_add fonksiyonunu kullanarak overflow kontrolü yapma.

overflowing_sub fonksiyonu alt taşmaları tespit eder. 
Ayrıca overflowing_mul, overflowing_div ve overflowing_pow gibi diğer varyasyonlar da mevcuttur, her biri uygun şekilde adlandırılmıştır.

# Notasyonlar

Ondalık taban, tamsayı değerler için varsayılan tabandır. 
Ancak, uygun gösterimle tabanı değiştirebilirsiniz:

- İkili için 0b kullanın.
- Sekizli için 0o kullanın.
- Onaltılı için 0x kullanın.

Çeşitli taban gösterimleri:

```
    println!("{}", 10); // 10
    println!("{:04b}", 0b10); // 0010
    println!("{}", 0o12); // 10
    println!("{}", 0xA); // 10
    println!("{}", 0xb); // 11
    println!("{}", 0xc); // 12
```

Bu gösterimler sayesinde, farklı sayı sistemlerindeki değerleri kolayca temsil edebilirsiniz. 
Örneğin, 0b10 ikili sistemde 2'yi, 0o12 sekizli sistemde 10'u, 0xA ise onaltılı sistemde 10'u temsil eder. 
Bu şekilde, farklı sayı sistemlerindeki değerler arasında dönüşümler yapabilir ve programınızın esnekliğini artırabilirsiniz.
Her bir taban, belirli bir sistemde sayıları temsil etmek için kullanılır ve doğru gösterim kullanılarak istenilen tabandaki sayıları elde edebilirsiniz.

# Floating Point Tipi

Gerçek sayılar için, Rust'ın IEEE 754 standardına uygun olarak tek ve çift hassasiyetli öncül türleri bulunmaktadır. 
Her tür bir işaret, üs ve mantissa bileşeninden oluşur. 
f32 türü, 32 bit genişliğinde olan tek-hassasiyetli sayıları temsil eder. 
f64 türü ise 64 bit genişliğinde çift-hassasiyetli sayıları temsil eder. Tür çıkarımı için, varsayılan kayan nokta türü f64'tür.
Integer tiplerinin aksine, float tipleri ya da floats, her zaman işaretlidir.

3_6 Kod Listesi, bir float örneğini göstermektedir.

-> Kod Listesi 3_6. Float Örneği.

Ne f32 ne de f64 tipi, sabit noktalı sayılar için ideal değildir. 
Bu özellikle, tam kesinlik önemli olan para birimi değerleri için geçerlidir. 
Kaybolan dolarlar ve kuruşlar birikmeye başlayabilir! 
rust_decimal kütüphanesinde bulunan Decimal türü, sabit noktalı float sayıları için harika bir türdür. 
rust_decimal kütüphanesine crates.io deposunda ulaşabilirsiniz. 
3_7 Kod Listesi'nde gösterildiği gibi from_str constructor veya dec! macro ile bir Decimal sayısı oluşturabilirsiniz.

-> Kod Listesi 3_7 Decimal Oluşturma

Bu şekilde, float tipleriyle hassas sayıların işaret, üssel bileşenlerini kullanarak işlemler yapabilir 
ve para birimi gibi sabit noktalı sayıları temsil etmek için Decimal türünü kullanabilirsiniz. 
Bu türlerin önemi, programınızın doğruluğunu ve hassasiyetini artırabilir.

# Floating Point Sabitleri

Kullanımınız için bilinen floating point sabitleri mevcuttur.
Bu sabitler, `std::f64::consts` modülünde f64 veri türü olarak uygulanmıştır. 
Aşağıdaki tablo, bazı f64 sabitlerinin bir listesidir.

Tablo: Bazı f64 Sabitlerinin Listesi

```
Ad             Açıklama         Değer
E              Euler sayısı     2.7182818284590451f64
FRAC_1_PI      1/π              0.31830988618379069f64
FRAC_1_SQRT_2  1/sqrt(2)        0.70710678118654757f64
FRAC_2_PI      2/π              0.70710678118654757f64
FRAC_2_SQRT_2  2/sqrt(2)        1.1283791670955126f64
LN_10          ln(10)           2.3025850929940459f64
LOG10_2        Log10(2)         0.3010299956639812f64
PI             ∏                3.1415926535897931f64
SQRT_2         sqrt(2)          1.4142135623730951f64
TUA            2π               6.2831853071795862f64

Bu sabitlerin kullanımı, matematiksel hesaplamalarda ve diğer alanlarda size kolaylık sağlayabilir.
Görsel programlama ve veri analizi gibi alanlarda özellikle yararlı olabilirler.
```

# Sonsuzluk

Rust, sonsuzluğu ve negatif sonsuzluğu destekleyen 32 bit ve 64 bit sürümlerini destekler. I
NFINITY ve NEG_INFINITY, std::f32 veya std::f64 modülünde bulunan const değerleridir. 

İşte iki örnek:

```
let space: f32 = f32::INFINITY;
let stars: f64 = f64::INFINITY;
```

-> 3_8 kod örneği, INFINITY sabitini kullanan bir örneği göstermektedir.

Rust dilinde sonsuzluk kavramını kullanarak matematiksel işlemleri daha esnek bir şekilde gerçekleştirebilirsiniz.
Bu özellik, belirli durumları kontrol etmek veya özel durumlar için işlemler yapmak için oldukça kullanışlı olabilir.

# NaN

Not a Number (NaN), sayısal olarak tanımsız veya bilinmeyen bir sonucu temsil eder. 
Bu bazen INFINITY değerlerini içeren formüllerin sonucu olabilir. 
Ayrıca, negatif bir sayının karekökü alındığında da NaN elde edilebilir.

-> 3_9 Kod örneği. Bu örnekte, 0.0 sayısının karekökünü alarak NaN durumunu kontrol ediyoruz. 
Eğer sonuç NaN değilse, karekök değerini ekrana yazdırıyoruz; aksi takdirde geçersiz bir sonuç olduğunu belirten bir mesajı ekrana yazdırıyoruz. 

Rust dilinde NaN kavramını bu şekilde kullanarak sayısal işlemleri daha güvenli bir şekilde gerçekleştirebiliriz.

# Sayısal Aralıklar

Rust dilinde tür boyutları diğer bazı dillere göre daha belirgin olduğundan,
geliştiriciler ihtiyaçlarına daha uygun ve verimli uygulamalar oluşturabilirler.
Aşağıdaki tablolar, imzalı ve imzasız tamsayı türlerinin minimum ve maksimum sınırlarını göstermektedir.

**Tablo 1: İmzalı Tamsayı Türleri Değer Aralıkları**
```
- i8: 8-bit -128 ile 127 arası
- i16: 16-bit -32768 ile 32767 arası
- i32: 32-bit -2147483648 ile 2147483647 arası
- i64: 64-bit -9223372036854775808 ile 9223372036854775807 arası
- i128: 128-bit -170141183460469231731687303715884105728 ile 170141183460469231731687303715884105727 arası
- isize: İşaretçi büyüklüğünde, mimariye bağlı
```

**Tablo 2: İmzasız Tamsayı Türleri Değer Aralıkları**
```
- u8: 8-bit 0 ile 255 arası
- u16: 16-bit 0 ile 65535 arası
- u32: 32-bit 0 ile 4294967295 arası
- u64: 64-bit 0 ile 18446744073709551615 arası
- u128: 128-bit 0 ile 340282366920938463463374607431768211455 arası
- usize: İşaretçi büyüklüğünde, mimariye bağlı
```

**Tablo 3: Ondalıklı Sayı Türleri Değer Aralıkları**
```
- f32: 32-bit -3.4 × 10^38 ile 3.4 × 10^38 arası (Yaklaşık)
- f64: 64-bit -1.8 × 10^308 ile 1.8 × 10^308 arası (Yaklaşık)
```

Çalışma zamanında sınırların kontrolü için, MIN ve MAX, belirli türlerin minimum ve maksimum değerlerini döndüren ilişkilendirilmiş sabitlerdir.
Aşağıdaki örnek bunu göstermektedir.

```
println!("{} {}", u32::MIN, u32::MAX);
println!("{} {}", f32::MIN, f32::MAX);
```

Bu bilgilerle, Rust dilinde türlerin belirli sınırlarını anlayabilir ve uygulamalarınızı daha etkili bir şekilde tasarlayabilirsiniz.

# Tür Dönüşümleri

Bir değeri, değişkeni veya sabiti mevcut türünden başka bir türe dönüştürebilirsiniz. 
Rust, örtük tür dönüşümleri için geniş destek sağlamaz. 
Birçok dil, hassasiyet kaybı olmadığında örtük dönüşümlere izin verir. 
Ancak Rust, bu durumda bile açık dönüşüm gerektirir. 

Türler Arasında Dönüşüm,

```
let degisken1 = 1i8;
let degisken2: f32 = 123.45;
let degisken3 = degisken2 as f64;
let degisken4 = 1.23 as u8 as char;
```

Önceki kod, izin verilen bir çift dönüşüm içerir. Örnekte bir float değerini doğrudan bir karakter değerine dönüştüremediğiniz için, 
float değeri önce işaretsiz bir tamsayıya dönüştürülür. Ardından işaretsiz tamsayı bir karaktere dönüştürülür.

float değerinden tam sayıya dönüşüm yaparken float sayısının ondalık kısmı kesilir.

Sayısal sabitler, tür son ekleriyle açıkça dönüştürülebilir.
Belirtim şu şekildedir: degerTipi, burada deger bir sayısal sabit ve Tip, i32, u32, f64 gibi tam tür adıdır, aşağıda gösterildiği gibi.
Diğer dillerde yaygın olan tek harfli son eklerin Rust'ta desteklenmediğini unutmayın. Rust'taki yaklaşım daha açıklayıcıdır.

```
let deger1 = 10i8;
let deger2 = 20f64;
```

Ayrıca bakınız,

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>()) 
}

fn main() {
    let deger = -3213213f64;

    print_type_of(&deger);

}
```
# Boolean Türleri

Rust'ta Boolean türü bool'dur ve mantıksal değerleri temsil eder. 
Bir bool türü için kabul edilebilir değerler true ve false'dur.

```rust
let kosul: bool = true;
```

Dahili olarak, bool değerleri mantıksal değerleri 0x01'e (true) ve 0x00'a (false) dönüştürülür.
Bool değerlerini i8 türlerine dönüştürebilirsiniz.
True, 1 olurken, false 0 olur.

Bool değerleri genellikle if anahtar kelimesi içinde kullanılır.
3_10'da örnek var.

# Char Türü

Char türü, tekil karakterler için kullanılır. 
Bu, bir Unicode karakteridir, UTF-8 kodlamasıyla Unicode Skaler Değeri (USV) şeklindedir. 
Ayrıca, char değerleri 4 bayt boyutundadır ve Unicode tablosundaki bir kod noktasını temsil eder.
Bu, alfasayısal karakterleri, kaçış karakterlerini, sembolleri ve hatta emojileri içerir.
Evet, Unicode artık gülücük yüzlerine de sahip! 
Unicode, dünya çapındaki çeşitli yazı sistemlerindeki her karakter için bir kod noktasına sahip olup, uluslararasılaştırmayı destekler.
ASCII, Unicode tablosunun başında yer alır.

Tek tırnaklar içinde bir literel char değeri nasıl tanımlayacağınızı gösterir,

```rust
let degisken1 = 'a';
let degisken2: char = 'b';
```

Doğru bir Unicode değeri varsayarak, bir tamsayı ile char türü arasında dönüşüm yapabilirsiniz,
aşağıdaki gibi gösterildiği gibi.

```rust
let degisken1 = 65 as char; // 'A'
let degisken2 = 'A' as i32; // 65
```

Unicode, çeşitli kaçış karakterlerini destekler. Bunlar genellikle gizli karakterlerdir, örneğin bir yeni satır veya sekme karakteri.
Aşağıdaki Tablo, her bir kaçış karakterini sunmaktadır.

```
Kaçış                Türü                         Açıklama
\n                   ASCII                        Yeni satır
\r                   ASCII                        Taşıma işareti
\t                   ASCII                        Sekme
\0                   ASCII                        Null
\\                   ASCII                        Ters Eğik Çizgi
\x{nn}               ASCII                        7-bit karakter kodu
\x{nn}               ASCII                        8-bit karakter kodu
\u{nnnnnn}           Unicode                      24-bit Unicode karakter
\'                   Alıntı                       Tek tırnak
\"                   Alıntı                       Çift tırnak
```

# Pointers(İşaretçiler)

Rust'ın iki tür işaretçisi vardır: referanslar ve raw işaretçiler.
Referanslar güvenli işaretçilerdir. Diğer yandan, raw işaretçiler basitçe bellekte bir değeri işaret eder.
İşaretçinin artık kullanılamadığı durumda belleği serbest bırakma gibi varsayılan davranış yoktur. 
Bu, C tarzı işaretçilere benzer.

Bir raw işaretçinin değeri bir bellek adresidir. Bu, yığın, bellek veya statik bellek konumundaki bir değeri gösteren bir bellek adresi olabilir. 
Raw işaretçiler kendileri bellekte bazı yerlerde bulunur ve genellikle yığında bulunur. 
Ancak her zaman değil - örneğin, bir node of a link listesin de depolanan bir raw işaretçi yığında olmayabilir.
Raw işaretçinin boyutu (yani, bellek adresi), ana sistem mimarisine bağlıdır.

Raw işaretçilerin, referansların ve düzenli değerlerin farklı türler olduğunu kabul etmek önemlidir. 
Bu ince ama önemli bir ayrımdır. Örneğin, i32 ve &i32 farklı türlerdir.
i32 türü, 32 bitlik bir tamsayı değerine işaret eder.
Bununla birlikte, &i32, bir i32 değerine güvenli bir işaretçiye işaret eden bir referanstır. 
Ayrıca, &i32, bir *i32 türünden farklıdır, bu da bir raw işaretçi türüdür.
İstenirse, her zaman bir referansı benzer bir türdeki bir raw işaretçiye dönüştürebilirsiniz.

“pa” ve “pb”, a ve b değerlerine raw işaretçilerdir:

```rust
let a: i32 = 10;
// 10
let b: i32 = 20;
// 20

let pa = &a as *const i32;
// 0x7fff149df274
let pb = &b as *const i32;
// 0x7fff149df278
```

İşaretçiler, Rust dilinde birinci sınıf vatandaşlardır ve diğer tüm türlerle aynı yeteneklere sahiptir.
İşaretçiler, değişkenler gibi kullanılabilir, yapı alanları, fonksiyon parametreleri veya dönüş değerleri olabilir.

# References(Referanslar)

Referanslar güvenli işaretçilerdir. Referanslar için belirli kurallar geçerlidir, fakat raw işaretçiler için geçerli değildir, bu kurallar güvenliği sağlamak için uygulanmıştır.
Referanslar, refere edilen bellek konumundaki değeri ödünç alırlar.
Referanslar hakkında akılda tutulması gereken diğer önemli noktalar şunlardır:

- Referanslar null olamaz.
- Temel değer geçerli bir tipte olmalıdır.
- Referansların ömürleri vardır.
- Referanslarla ilişkili sahiplik gibi davranışlar bulunmaktadır.

Referans türünü, daha önce gösterildiği gibi, bir & ile bildirirsiniz. 
Ayrıca, bir değeri referans almak için (&) operatörünü kullanabilirsiniz. 
Bu bağlamda ampersand işaretini "referansını al" olarak yorumlayabilirsiniz.

Dereference operatörü (*) bir referansı dereference eder. 
Referans yapılan bellek adresindeki değeri alırsınız. 
Bu, referansın temel değeridir. 

Örneğin, aşağıdaki ayrıştırmanın bir örneğini:

```rust
let aref: &i32 = &5;
let value: i32 = *aref;
```

Matematik operatörleri hem değer hem de referans türleri için uygulanır. 
Bir referans için matematiksel bir hesaplama yapılmadan önce, değer otomatik olarak dereference edilir.

Referans ve matematik operatörleriyle ilgili örnekler:

```rust
let ref1 = &15;
let ref2 = &20;
let value1 = ref1 + 10;
let value2 = ref1 * ref2;
println!("{} {}", value1, value2); // 25 300
```

Yukarıdaki bildirimleri value1 ve value2 için farklı şekilde yeniden yazabilirsiniz, ancak sonuç aynı olacaktır. 
Ancak, aşağıdaki kod okunması daha zordur ve gereksiz dereferans yapar. 
Matematiksel işlemler için otomatik olarak yapılacaktır.

```rust
let value1 = *ref1 + 10;
let value2 = *ref1 * *ref2;
```

Referanslar, dereference işlemi ve matematik operatörlerini içeren bir örnek, 3_12'de.

`==` operatörü, referansta bulunan değerleri karşılaştırır, bellek konumlarını değil. 
Eğer gerçek bellek adreslerini karşılaştırmak istiyorsanız, 
std::ptr modülündeki eq fonksiyonunu çağırabilirsiniz.

Örnek 3_13'de...

Ayrıca ikiden fazla referans'ı karşılaştırmak için şu şekil de yapabilirsiniz:

```rust
use std::{env::consts, ptr};

fn main() {
    let dolar = 30;
    let euro = 30;
    let ruble = 30;
    let sterlin = 30;

    
    let dolar = &dolar;
    let euro = &euro;
    let ruble = &ruble;
    let sterlin = &sterlen;


    let references = [dolar as *const i32, euro as *const i32, ruble as *const i32, sterlin as *const i32];
    
    let are_equal = references.iter().all(|&x| x == references[0]);
    
    println!("Bütün referansların karşılaştırma sonucu: {}", are_equal);
}
```
