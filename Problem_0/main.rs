fn main() {
	let mut sum: i64 = 0;
	for number in 1..738000 {
		if number % 2 == 1 {
			sum += number * number;
		}
	}
	println!("{}", sum);
}
