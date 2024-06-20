#[warn(unused_mut)]

fn main() {

    let (bir, iki, üc, dört) = (1, 2, 3, 4);
    let sonuc = bir + iki + üc + dört;

    println!(
        "{ad} {bir} + {iki} + {üc} + {dört} = {sonuc}",
        ad = "Ortalama: ", sonuc = sonuc as f32 / 2.0
    );

}
