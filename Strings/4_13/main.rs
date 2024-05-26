fn func_deref(str_1: &str) {

    println!("{}", str_1);

}

fn main() {

    let string_1 = "Merhaba".to_string();
    func_deref(&string_1);

}
