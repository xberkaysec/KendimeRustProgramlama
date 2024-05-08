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

