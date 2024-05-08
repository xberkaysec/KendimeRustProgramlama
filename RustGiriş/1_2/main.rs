fn check_even_odd(num: i32) {
    match num {
        n if n % 2 == 0 => println!("{} çifttir.", n),
        _ => println!("{} tektir.", num),
    }
}

fn main() {
    check_even_odd(5);
    check_even_odd(10);
}
