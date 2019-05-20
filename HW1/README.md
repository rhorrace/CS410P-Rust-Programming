Homework 1 Report
=======

## Robert Horrace

For homework 1, a calc program  
was created to have a sum function,  
a prod function, a gcd function, and  
a lcm function.

The sum function was implemented  
to return the 1st argument plus the 2nd  
argument 2. The prod function was  
implemented to return the 1st argument  
times the 2nd argument. These were the  
simplest functions to implement, due to  
the idea of the sum and product of two  
numbers.

The gcd function was a little tricky.  
The example in the book does not work  
if the two arguments passed in are both  
zero, the function panics, however I  
modified the function to actually return  
a value when both arguments are zero.  
If both argments m and n are zero, the function  
returns zero, which is mathematically correct.  
If n is zero, the m is returned, otherwise  
a loop is initiated and repeats until m is  
not zero. M and n are swapped if m is  
less than n and then m become m mod n.

The lcm function was somewhat easier.
The arguments m and n are passed in,  
but they are not mutable, due to the  
algorithm of the lcm function. Temporary  
variables a and b were made and set to m  
and n. Now, if a or b are either zero,  
zero is returned. The function continues  
and a loop is initiated, which repeats  
until a equals b. In the loop, if a < b,
a becomes a + m, otherwise b becomes b + m.  
When the loop ends, either a or b can be  
returned, so a was chosen as the returned  
value.

A main function was implemented to handle  
dealing with the four functions implemented.  
The main reads the user's arguments and separates  
the argument to a string containing the name of  
a function and a list of numbers. If the  
name is invalid, then an error is  
displayed and main exits. If the list of  
numbers is empty, then zero is displayed  
and main exits. Now, the name of the function  
will determine what function is executed.  
The function will execute until all numbers  
in the list of numbers have been used.
The result will be printed when the function  
is done.

For testing, test functions for testing the  
sum, prod, gcd, and lcm function were implemented.
The test cases were set and all tests are designed  
to pass for all cases. After testing the cargo. All  
tests passed.

For testing main, running the program manually seemed  
to be the best option. Running the program with zero  
arguments gave an error, as expected, running the program  
with a made-up function displayed an error, running the  
program with a valid function and zero number arguments  
displays a zero, running a valid function with one or more  
number arguments displays a number depending on the function,
like sum 1 2 displays 3.

Test examples:

	* calc -> nothing
	* calc foo -> Error
	* calc sum  -> 0
	* calc prod -> 0
	* calc gcd  -> 0
	* calc lcm  -> 0
	* calc sum 1 2  -> 3
	* calc prod 1 2 -> 2
	* calc gcd 1 2  -> 1
	* calc lcm 1 2  -> 2
	* calc sum 1 2 3  -> 6
	* calc prod 1 2 3 -> 6
	* calc gcd 1 2 3  -> 1
	* calc lcm 1 2 3  -> 6

Implementing the program went rather well.  
Implementing the functions were relatively 
easy, but implementing the main was a bit  
difficult, due to dealing with Rust's method  
of arrays and vectors. Dealing with strings  
was a bit tricky and remembering how important  
mut can be sometimes. Overall, the program went  
well and it does the job.

---
