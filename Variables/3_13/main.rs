use std::ptr;

fn main() {
    
    let dolar = 30;
    let euro = 30;
    
    let dolar = &dolar;
    let euro = &euro;

    let t = dolar == euro;

    println!("{}", t);

    let f = std::ptr::eq(dolar, euro);

    println!("{}", f);
    
}
