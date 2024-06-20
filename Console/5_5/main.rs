#[warn(unused_mut)]

fn main() {

    let (ilk, iki, üc, dört) = (1, 2, 3, 4);
    
    println!(
        "{} + {} + {} + {} = {sonuc}", 
        ilk, iki, üc, dört,
        sonuc = ilk + iki + üc + dört
    );
}
