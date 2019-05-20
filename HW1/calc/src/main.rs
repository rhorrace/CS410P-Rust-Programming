/*
	Trademark © Robert Horrace (I don't remember and know how to properly trademark it)
	This is the Rust program for HW1.
	It has a sum, prod, gcd, and lcm
	function, and they are called
	depending on what the user wants.
	The arguments will be the function
	name a n-amount of numbers to be
	added, multiplied, gcd'd, or lcm'd.
*/

use std::str::FromStr;
use std::env;
use std::mem;

/*Sum of two numbers*/
fn sum(n: u64, m: u64) -> u64 {
	n + m
} 

/*Test sum function*/
#[test]
fn test_sum() {
	assert_eq!(sum(0,0),0);
	assert_eq!(sum(0,1),1);
	assert_eq!(sum(1,2),3);
	assert_eq!(sum(2,1),3);
}

/*Product of two numbers*/
fn prod(n: u64, m: u64) -> u64 {
	n * m
}

/*Test prod function*/
#[test]
fn test_prod() {
	assert_eq!(prod(0,0),0);
	assert_eq!(prod(0,1),0);
	assert_eq!(prod(1,0),0);
	assert_eq!(prod(1,1),1);
	assert_eq!(prod(1,2),2);
	assert_eq!(prod(2,1),2);
}

/*GCD of two numbers*/
fn gcd(mut n: u64, mut m: u64) -> u64 {
	if n == 0 {
		return m;
	}
	while m != 0 {
		if m < n{
			mem::swap(&mut m, &mut n);
		}
		m %= n;
	}
	n
}

/*Test gcd function*/
#[test]
fn test_gcd() {
	assert_eq!(gcd(0,0),0);
	assert_eq!(gcd(0,1),1); 
	assert_eq!(gcd(1,0),1);
	assert_eq!(gcd(1,2),1);
	assert_eq!(gcd(2,1),1);
	assert_eq!(gcd(2,4),2);
	assert_eq!(gcd(4,2),2);
}

/*LCM of two numbers*/
fn lcm(m: u64, n: u64) -> u64 {
	let mut a = m; let mut b = n;
	if a == 0 || b == 0 {
		return 0;
	}
	while a != b {
		if a < b {
			a += m;
		}
		else {
			b += n;
		}
	}
	a
}

/*Test lcm function*/
#[test]
fn test_lcm(){
	assert_eq!(lcm(0,0),0);
	assert_eq!(lcm(0,1),0);
	assert_eq!(lcm(1,0),0);
	assert_eq!(lcm(1,1),1);
	assert_eq!(lcm(1,2),2);
	assert_eq!(lcm(2,1),2);
}

/*Main function*/
fn main() {
	/*variable declarations*/
	let flag;																											//Dependent on function name in cmd line 
	let mut x;																										//For operations
	let mut cmdargs: Vec<String> = env::args().skip(1).collect(); //for capturing cmd line argumends

	/*If cmdargs is empty*/
	if cmdargs.len() == 0 {
		std::process::exit(1);
	}

	let function: String = cmdargs.remove(0); 										// For function name (should be first argument)
	let mut numbers = Vec::new(); 																//For the numbers in cmd line 

	
	/*change flag depending the on the function name*/
	match function.as_ref() {
		"sum"  => flag = 1,
		"prod" => flag = 2,
		"gcd"  => flag = 3,
		"lcm"  => flag = 4,
		      => flag = 0,
	}
	
	/*Displays Error and exits early if the flag is 0*/
	if flag == 0 {
		println!("Error: not a valid function");
		std::process::exit(2);
	}
	
	/*If there are no numbers, display 0 then exit*/
	if cmdargs.len() == 0 {
		println!("{}",0);
		std::process::exit(3);
	}

	/*For placing the the numbers in cmdargs(strings) to numbers (u64)*/
	for arg in cmdargs {
		numbers.push(u64::from_str(&arg).expect("error parsing argument"));
	}
	
	/*Start one of the operations dending on flags*/
	x = numbers[0];
	if flag == 1 {						//If sum is the function wanted
		for m in &numbers[1..] {	
			x = sum(x, *m);
		}
	}
	else if flag == 2 {				//If prod (product) is the function wanted
		for m in &numbers[1..] {
			x = prod(x, *m);
		}
	}
	else if flag == 3 {				//If gcd is the function wanted
		for m in &numbers[1..] {
			x = gcd(x, *m);
		}
	}
	else {										//If lcm is the function wanted
		for m in &numbers[1..] {
			x = lcm(x, *m);
		}
	}

	/*Print the result*/
	println!("{}",x);
}
