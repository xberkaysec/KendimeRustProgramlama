/**
Bu kod, farklı diller için farklı selamlama mesajlarını içerir:

- "enUS" için: "Hello, world!"
- "enUK" için: "Good day, world!"
- "frFR" için: "Bonjour le monde!"
- "hiIN" için: "नमस्ते दुनिया!"
- Diğer diller için: "Hello, world!" (varsayılan durum)
**/

pub fn hello_world(language:&str)->&'static str {
    match language {
        "enUS"=>"Hello, world!",
        "enUK"=>"Good day, world!",
        "frFR"=>"Bonjour le monde!",
        "hiIN"=>"नमस्ते दुनिया!",
        _=>"Hello, world!"
    }
}
