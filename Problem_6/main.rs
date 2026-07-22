fn main() {
	let mut sum_sq:i64 = 0;
	let mut sum:i64 = 0;
	for i in 1..=100 {
		sum_sq += i * i;
		sum += i;
	}
	println!("{}", sum * sum - sum_sq);
}
