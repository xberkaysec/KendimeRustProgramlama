#[warn(unused_mut)]

fn main() {

    
    let (bir, iki, üc, dört) = (1, 2, 3, 4);
    let sonuc = bir + iki + üc + dört;

    println!(
        "{3} + {2} + {1} + {0} = {4}",
        bir, iki, üc, dört, sonuc
    );

}
