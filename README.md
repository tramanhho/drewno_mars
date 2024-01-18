<html>

<center>

<img src="https://img.shields.io/badge/Rust-v1.75.0-coral?logo=rust" /> &nbsp;&nbsp;&nbsp;&nbsp;
<img src="https://img.shields.io/badge/Unit%20Tests-Passing-limegreen?logo=github" /> &nbsp;&nbsp;&nbsp;&nbsp;
<img src="https://img.shields.io/badge/Integration%20Tests-3%2F7%20Passing-orange?logo=github" /> &nbsp;&nbsp;&nbsp;&nbsp;
<img src="https://img.shields.io/badge/codecov-77.20%25-orange?logo=codecov" />
</center>
</html>

## About

Drewno Mars is a C++-like toy language that supports a variety of features, like console input/output, functions, multiple variable types, loops, and more. It is built in Rust to front-load processing time to compilation, which maximizes runtime efficiency. 

## Language Specifications

* Console output - denoted by the `give` keyword   
  * `give [variable | string]`
  * `give "Hello World"`
* Console input - denoted by the `take` keyword   
  * `take [variable]`
  * `take integer`
* Variable declaration - denoted with `:`, strongly typed
  * `[variable_name] : [type]` 
  * `b : bool`
* Constants - denoted by the `perfect` keyword
  * `i : perfect int`
* Statements - ends with a `;`
* Functions - no keyword
```
[function_name] : [return_type] (args...) {
	[statement];
	[statement];
	...
}
```
```c#
main : void () { 
	give "Hello World\n";
}
```
* Conditionals

```
if ([condition]) { 
	[statement];
	[statement];
	...
} else {
	[statement];
	[statement];
	...
}
```
```c#
if (true) { 
	give "True"; 
} else { 
	give "False"; 
}
```

## Example code 

```c#
n : int;
b : bool;

main : () void {
    n = 1;
    b = true;
	
	while (n < 10) {
		n = n + 3;
	}

    give n;
    give "\n";
}
```

Classes

```c
Animal : class {
	[field variable declaration];
	[field variable declaration];
	[field variable declaration];
	...
}
```

```c#
Animal : class {
	age : int;
}

main : () void {
    a : Animal;
	a--age = 15;
	give a--age;
}
```

## Language Implementation

Scanning and Parsing is done with a combination of [Logos](https://docs.rs/logos/latest/logos/) and [LALRPOP](http://lalrpop.github.io/lalrpop/). Type/Name Analysis is static and covers a variety of errors, including but not limited to: 

* Invalid/Multiple declarations
* Accessing an undeclared variable
* Incompatible types in operations 
* Missing/incompatible return types
* Invalid arguments in function calls

Drewno Mars compiles down to x86 Assembly, and utilizes libc function to implement console input/output, as well as random bool generation.

## Acknowledgements
- Drew Davidson for generating the template languages specifications and libc integrations under `stddrewno_mars.c`