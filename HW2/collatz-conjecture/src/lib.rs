pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
			return None;
		}
		let mut x:     u64 = n;
		let mut count: u64 = 0;
		while x != 1 {
			if x % 2 == 0 {
				x /= 2;
			} else {
				x = 3 * x + 1;
			}
			count += 1;
		}
		Some(count)
}
