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
-> Kod Örneği 3_1. isize boyutunu doğrulama kodu.

Önerildiği durumlarda, işaretli tamsayı türünün varsayılan tipi i32'dir. 
İşaretsiz tamsayılar için ise varsayılan tip u32'dir. 
Öntanımlı bir tamsayı değerinin varsayılan türünü gösteren Kod Örneği 3_2. 
Bu kod, genel kavramlar, Insight(içgörü) ve any türünü içerir. 

Daha iyi okunabilirlik için, sayısal literallerde ve ondalık sayılar arasında alt çizgiler kullanılabilir. Genellikle alt çizgiler, 103'lük segmentleri belirtir. Ancak, Rust alt çizgi polisi değildir. Alt çizgiyi sayının herhangi bir yerine koyabilirsiniz, Kod Örneği 3.4'te gösterildiği gibi.
Kod Örneği 3.4. Sayılarda ayırıcı olarak alt çizgiler eklemek

let normal1 = 123_456_678;
let normal2 = 123_456.67;
let ilginç = 12_3_456;
