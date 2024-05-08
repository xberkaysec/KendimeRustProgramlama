fn factorial(n:i32) -> i32 {
	match n {
		0..=1=>1,
		_=> n*factorial(n-1)
	}
}

fn main() {
	let num = 4;
	let result = factorial(num);
	println!("{} sayısının faktöriyeli: {}", num, result);
}

// Çıktısı - 4 sayısının faktöriyeli: 24 
