# Basic concepts

## Variables

Rust variables are immutable by default to illustrate this we can use the following code:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

checking this code with `cargo check`  shows us that this code doesn't even compile. Instead it gives us the error "Cannot assign twice to immutable variable". The rust team clarifies that they've taken this decision to prevent unwanted behaviour. You have to specifically say that a variable can change in the future so that pieces of code don't break because they get unexpected input. Rust also aims to provide as much as possible compile time errors (Errors you get while compiling the program) to prevent any mistakes from slipping your conscious and being difficult to track later down the line.

You can specify a variable can change by adding the mut keyword in front of it so our code example from above would be:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Another reason to specify mutability is because it can be faster to mutate data in large data structures but it also promotes a more functional programming style. Making new data structures can sometimes be way easier to understand. So don't fight the immutability work with it.

## Constants

Another concept within Rust is constants instead of variables they are immutable as well but they can't be modified to be mutable. This is mainly used instead of hardcoded values to have one place to maintain them and convey their meaning for other people maintaining your code. To define a constant we use the const word instead of the let word:

```rust
const SPEED_OF_LIGHT = 9.81;
```

The naming convention is all uppercase with a _ in between words.

Also note that you can use _ in numerical values to improve readability like so:

```rust
const BIG_ASS_NUMBER = 100_000_000;
```

## Compound variables

Rust has a way of grouping different values together in a structure called a tuple. A tuple doesn't have to contain the same types for example this is a valid tuple definition:

```` rust
let tup: (&str, u8,i8,char) = ("test",1,-1,'z');
````

A neat feature of rust is you have two ways of getting back these values either trough pattern matching or indexing (starting at zero):

```rust
let tup: (&str, u8,i8,char) = ("test",1,-1,'z');
let (x,y,z,a) = tup;
let second_element = tup.1;
```

also note the &str I asked why it was a borrowed string on discord and got the following answer:

When you compile the program, it creates an executable file. String literals are included exactly the way you typed them in the executable. The part of it where they're placed is called *static memory* References to them are `&'static str` and borrow from the very executable file that your program is.(edited)

## Arrays

Arrays are very similar to arrays in other languages they're not allowed to grow and have a fixed length. They do require all the elements to be off the same type. A vector is similar to an array but it's allowed to grow and shrink. To specify the type of an array you use the syntax *[type;length]*. 

```rust
let ar: [u8,3] = [1,2,3]
```

You can also use a short hand syntax if you want to initialize an array with all the same elements

```rust
let ar : [u8;3]= [1;3]
```

this would make an array with 3 1's in it.

## Functions

The convention name for functions are snake case. all lowercase and _ separated words. Functions are defined by the fn keyword followed by the name, parantheses, arguments and the curly braces just like any other language:

```rust
fn another_function(){
	println!("hello from function 2");
}

fn main(){
	println!("Hello world");
	another_function();
}
```

Notice that it doesn't matter where the function is defined after or before the main function.

