fn main() {

    let mut abc = "ac".to_string();
    abc.insert(1, 'b');

    println!("{}", abc);

    let mut bir_iki_üc = "bir üc".to_string();
    bir_iki_üc.insert_str(3, " iki");

    println!("{}", bir_iki_üc);

}
