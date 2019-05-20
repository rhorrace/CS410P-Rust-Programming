pub fn check(word: &str) -> bool {
	if word.is_empty() {
		return true;
	}
	let filtered: String = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
	let unique = |x: char, word: &str| word.find(x).unwrap() == word.rfind(x).unwrap();
	let filteredcount = filtered.chars().filter(|&x| unique(x,filtered.as_str())).count();
	if filtered.len() == filteredcount {
		return true;
	} else {
		return false;
	}
}
