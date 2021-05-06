# Guessing game

## Setting up

We'll be programming a guessing game in this chapter. To start we make a new project we can use the build in package manager cargo for this by running the command `cargo new guessing_game` . Just as a reminder to run our project we can use `cargo run`. By using cargo new we get our hello world for free pretty neat huh

```rust
fn main() {
    println!("Hello, world!");
}
```

## Getting input

So to make our number guessing game we need to take in user input. We do that with the following code:

```rust
use std::io;

fn main() {
    println!("Guess the number game");
    println!("Please input a number:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong with reading");
    println!("the guess is {}", input);
}
```

The first two statements we're already familiar with these are mainly the println macros we've seen earlier. Following those up we define a new input string. As you remember variables are immutable by default. So we need to make this mutable by adding the mut keyword. Also notice the :: syntax this is used to call a method on a type for example the String type has an associated method called new to make a new instance. This is sometimes refered to as a static method.

We also want to get a module for input output this is called the io module and is defined in the standard library. We include it by the use statement as you're familiar with in other languages. We call the read_line method on our standard input. This takes a variable where the input will be stored in. the & is used to indicate it's a reference to a variable so we don't have to copy our input variable multiple times into memory. References are also immutable by default so we add our &mut to it.

From read_line we get a so called result structure. This is imported in the prelude so we don't have to import it ourselves. The result is actually an enum with two different values either okay or error. The error contains a message while the operation failed while okay contains the value that's returned by the function. Normally you have to do proper error handling but there's also a short way of doing this with the `expect method` which in the case of okay will return the value or in case of error crash the program and return the message passed in.

The syntax of string interpolation I've already discussed in the first project we've done together

## Cargo

We need to import the random crate because this is not included default in the rust library. So we need to import it from somewhere. Rust uses crate as it's package manager. In our cargo.toml file there is a section called dependencies. We can add our reference to our crate here 

```toml
[dependencies]
rand = "0.8.3"
```

By running cargo.build our crate will automatically be downloaded. Please note that this uses semantic versioning so this will automatically download the most recent minor crate so everything above 0.8.3 and below 0.9.0.  All of the rust public available libraries or crates you can find on [crates.io](https://crates.io/). Obviously by running cargo build the dependencies that rand has will  also be downloaded.

Another cool feature is the so called **Cargo.lock** file this file will be created when the build is done for the first time. When all version numbers and dependencies have been figured out they will be written to the cargo.lock file. If this file exists then all versions from there will be taken so you have the same build process each time and across all machines. This way you can share code without having to worry about dependencies. This means that if the most recent version is 0.8.3 at time of download it will remain at 0.8.3 untill we explictely upgrade it.

Lastely to update our cargo file we will have to run the command `cargo update` this will update the dependencies to the lastest version note that if a new mayor version is released like 0.9.0 we will only update to the latest 0.8.x version. We need to manually update the version is the cargo.toml file to be 0.9.x.

The last command of cargo that's really usefull is `cargo doc --open` this will automatically make documentation of our project and provide us with an overview of all crates and their associated functions. We will go more in depth on documentation later.

## Randomness

So now we have our crate ready we include it in a use statement to bring it into scope with a use statement. We got all of this info from the documentation you found with cargo doc --open.

```rust
use rand;
```

 Now we can use a structure called thread_rng to get a new random number generator in a seperate thread:

```rust
rand::thread_rng()
```

Now that we have our structure that's capable of generating a random number we can generate it by calling the method gen_range. This method takes a range as an argument a range has the following syntax: x..y where x is inclusive and y is exclusive so to get a number between 1 and a 100 we use the following syntax

```rust
let rand_number:i32 = rand.thread_rng().gen_range(1..101);
```

We can then print this number on the screen with string interpolation:

```rust
println!("The secret number is {}", rand_number);
```

## Comparing the geuss and secret number

We have to compare the input and the randomly generated number. To do this we have a slight problem however, the randomly generated number is an integer an 32 bit integer and our input is a string. We have a parse method on string that actually converts it to a number and by specifying the type we can cast to an 32 bit integer (i32). This returns a result and we can handle it in much the same way by using expect. We however have another slight problem. We called read_line which forces the user to press enter appending *"\n"* to our string. We can eliminate this by calling trim which removes whitespace and enters from string (like many other languages). So the syntax to parse our string is the following:

```rust
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong with reading");
    let input:i32 = input.trim().parse().expect("Something went wrong parsing the string");
```

notice something quirky tho? We have two variables called input. This is however possible by using something called **shadowing**. This allows us to reuse the same variable name so we don't have to work with input_string and input variables like in many other languages.

Now that they both have the same type let's compare them. To compare them an int and many other types have what's called a compare function called cmp. This returns something called an ordering. You can look at the (documentation)[https://doc.rust-lang.org/core/cmp/enum.Ordering.html] to see what this is exactly. An Ordering is much like result an enum with a few possible values namely: Less, Equal, Great. We can use a match statement which is much like a switch statment to handle these enum values. However we need to import the ordering enum in order to use it in code. we can see it's contained within the core library in the cmp module:

```rust
use std::cmp::Ordering;
```

Now we can write our match statement like so:

```rust
    match rand_number.cmp(&input) {
        Ordering::Equal => println!("Congratulations you won!"),
        Ordering::Greater => println!("Too small!"),
        Ordering::Less => println!("Too big!")
    }
```

If we run this code we see that it works perfectly.

However we want to be able to loop this. Rust has a very intuitive way of doing this using a so called loop block. We can wrap all of our main code in this loop block. We can use a break statement to break out of our loop block:

```rust
    loop {
        println!("Please input a number.");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Something went wrong with reading");
        let input: i32 = input.trim().parse().expect("Something went wrong parsing the string");
        println!("the guess is {}", input);

        match rand_number.cmp(&input) {
            Ordering::Equal => {
                println!("Congratulations you won!");
                break;
            },
            Ordering::Greater => println!("Too small!"),
            Ordering::Less => println!("Too big!")
        }
    }
```

Since this is also the last block of code at the end of our main function our program automatically quits afterwards.

## Finishing up

A final thing we can do to improve our game is error handling an invalid input. Whenever we input a string in our read_line the parse method will return an exception and because we're using expect this will result in panic and thus crash the program. Now that we now how to use the match syntax and we know that Result is an enum we can use the following code. Please note that the continue statement is used to continue to the next iteration of our loop.

```rust
let mut geuss: String = String::new();
io::stdin().read_line(&mut geuss).expect("Something went wrong reading the value");
let geuss:i32 = switch geuss.trim().parse() {
    Ok(val) => val,
    Err(_) => continue
}
```



So our final code looks like this:

````rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number game");

    let rand_number:i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a number.");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Something went wrong with reading");
        let input: i32 = match input.trim().parse(){
            Ok(val) => val,
            Err(_) => continue
        };
        println!("the guess is {}", input);

        match rand_number.cmp(&input) {
            Ordering::Equal => {
                println!("Congratulations you won!");
                break;
            },
            Ordering::Greater => println!("Too small!"),
            Ordering::Less => println!("Too big!")
        }
    }
}

````

