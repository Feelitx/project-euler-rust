fn is_palindrome(x:i32) -> bool {
	let head:i32 = x / 1000;
	let tail:i32 = x % 1000;
	let mut cond:bool = head / 100 == tail % 10;
	cond = cond && head % 10 == tail / 100;
	cond = cond && (head/10)%10 == (tail/10)%10;
	if cond {return true;}
	false
}

fn main() {
	let mut m:i32 = 0;
	for i in 900..1000 {
		for j in 900..1000 {
			let mul:i32 = i * j;
			if mul > m && is_palindrome(mul) {m = mul}
		}
	}
	println!("{}", m);
}
