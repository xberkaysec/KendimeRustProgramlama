fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let deger = 1; // öntanımlı tür
    let deger2 = 1.1; // öntanımlı tür

    print_type_of(&deger);
    print_type_of(&deger2);

}
