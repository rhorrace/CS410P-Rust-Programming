pub fn find() -> Option<u32> {
	let sum:u32 = 1000;
	for a in 1..sum-2 {
		for b in a+1..sum-1 {
			for c in b+1..sum {
				if a*a + b*b == c*c {
					if a+b+c == sum {
						println!("{} {} {}",a,b,c);
						return Some(a*b*c);
					}
				}
			}
		}
	}
	None
}
