fn main() {
    let mut sum: i64 = 0;
    let mut first: i64 = 1;
    let mut second: i64 = 2;
    while second <= 4_000_000 {
        if second % 2 == 0 { sum += second; }
        second += first;
        first = second - first;
    }
    println!("{}", sum);
}