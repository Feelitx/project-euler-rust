fn main() {
	let mut num:i64 = 600_851_475_143;
	let mut i:i64 = 2;
	let mut m:i64 = 0;
	while i <= num.isqrt() {
		while num % i == 0 {num /= i;}
		m = i;
		i += 1;
	}
	if num > m {m = num;}
	println!("{}", m);
}
