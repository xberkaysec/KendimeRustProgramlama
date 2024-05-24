fn main() {

    let vector = vec![1, 2, 3, 4, 5];
    let string = String::from_utf8(vector).unwrap();

    println!("{:?}", string);

} 
