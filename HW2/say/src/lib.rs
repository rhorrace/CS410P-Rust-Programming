//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(mut num: i32) -> String {
	if num < 0 {
		return "won't compile".to_string();   // Returns "won't compile" string
	}	
	else {
		let mut result = String::new(); // For the result
		let places: [&str; 4]   = ["hundred","thousand","million","billion"]; // For the 100s & 1000(000(000))s
		let tens: [&str; 10]    = ["","","twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"]; // For the teens (20,30..90)
		let two_dig: [&str; 10] = ["ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"]; // For the teens (10-19)
		let ones: [&str; 10]    = ["zero","one","two","three","four","five","six","seven","eight","nine"]; // For the ones place
		if num == 0 {
			result.push(ones[0].to_string());
		}	
		else {
			let mut tmp = String::new();
			let mut place = 1;
			while num != 0 {
				let z: i32 = num % 10;                       // Ones places
				let y: i32 = ((num % 100) - z) / 10;         // Tens place
				let x: i32 = ((num % 1000) - y - z) / 100;   // Hundreds place
				if x != 0 {
					tmp.push(ones[x].to_string()).push(" ".to_string()).push(places[0].to_string());
				}
				if y == 0 {
					if z != 0 {
						
					}
				} 
				else if y == 1 {
				} 
				else {
					if z == 0 {
					} else {
					}
				}
				num /= 1000;
				if num != 0 {
					place += 1;
				}
			}
		}
		return result; // Return overall result in string form
	}
} 
