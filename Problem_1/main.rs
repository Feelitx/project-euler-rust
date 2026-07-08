fn main() {
	let mut sum3 = 0;
	let mut sum5 = 0;
	let mut sum15 = 0;

	for _n in (0..1000).step_by(3) {
		sum3 += _n;
	}

	for _n in (0..1000).step_by(5) {
		sum5 += _n;
	}

	for _n in (0..1000).step_by(15) {
		sum15 += _n;
	}
	
	println!("{}",sum3 + sum5 - sum15);
}
