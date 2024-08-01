fn main() {
    
    let (f1, f2) = (1.2, 3.45678);
    println!("Result: {:<10.2} {:.2}", f1, f2); 
    
    let (f1, f2) = (1.2, 3.45678);
    println!("Result: {1:<10.0$} {2:.0$}", 2, f1, f2); 

}
