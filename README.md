## About

This project is a compiler written in Rust for a C++-like toy language. This compiles down to x86 Assembly. 

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

Scanning and Parsing is done with a combination of [Logos](https://docs.rs/logos/latest/logos/) and [LALRPOP](http://lalrpop.github.io/lalrpop/). Type/Name Analysis is static. 
