# Rust Learning Project - Key value dictionary Part 1

We're going to build a key value database in Rust.
We need to start a new cargo project with the syntax cargo new kvstore. By default cargo will give us a binary application. Cargo will make a metadata file called cargo.toml in our main directory.

the main body of the code is in the main function

```rust
fn main(){
	println!("Hello world!");
}
```

How do we run a cargo package? well first we need to check the code we can do that with cargo by using `Cargo check` then we can use `cargo build` to make the executable. This is build in the target/debug folder since it's a debug release and not a build release. There's a long build for the release version you can build it with `cargo build --release`. If you want to build and run you can do `cargo run`. 

Cross compilation is possible with rustc so we can compile for different operating systems. 

How do we get the command line arguments passed to our application:

```rust
std::env::args()
```

The syntax is as follow each namespace has sub namespaces to so the standard library has a library environment where we're invoking the function args to  get the arguments passed into the command line.

The rust analyser automatically types the arguments itself if we put it in a variable

```rust
let args:Args = std::env::args();
```

Args is an iterator this means we can easily loop over all the arguments themselves. We can also easily skip over one element by calling the skip method

```rust
let args:Skip<Args> = std::env::args().skip(1);
```

we need to skip over the first element considering we're invoking our tool using the following syntax:

`kvstore hello world`

So the first argument would be the name of our application now we can get the first `hello` argument which is our key like this

```rust
    let mut args: Skip<Args> = std::env::args().skip(1);
    let key: Option<String> = args.next();
```

as we can see we had to add the mut keyword to args. This is because by default all of the variables in rust are immutable by default so we use the mut keyword to define that they can change.

So why don't we just get a String back with a null if we invoke the binary like this: `kvstore`  In most languages the key would now contain a null reference. Rust doesn't have the concept of null so that's why it's using the Option syntax. So we have to handle the null case ourself. We can unwrap the option so this will give it to us and if it's a null reference it will crash the program:

```rust
let key: String = args.next().unwrap();
```

if we don't pass in any arguments by doing `cargo run`  then we will get an exception like this:

![image-20210429164541365](C:\Users\gebruiker\AppData\Roaming\Typora\typora-user-images\image-20210429164541365.png)

If we pass in arguments (this is done by prefixing the arguments with -- when using cargo) we see that it runs successful:

`cargo run -- hello`.

We can now see if everything is running sucessfully:

```rust
    let key: String = args.next().unwrap();
    println!("the key is {:?}",key);
```

and we see that our output is "hello". String interpolation is done with these curly braces {}. I'm also specifying that it's meant to be run in debug mode so I can see that there's no excess spaces in the string by specifying :?.

Lets get the value now too:

```rust
let mut args : Args = std::env::args();
let key: String = args.skip(1).next().unwrap();
let value: String = args.next().unwrap();
println!("The key is {0:?} and the value is {1:?}", key,value);
```

How do we write a more friendly error message if our unwrap function crashes. There's a better function we can use if it goes wrong. namely expect, this change the error to represent a specific string:

```rust
let key: String = args.next().expect("The key was not there");
let value: String = args.next().expect("There was no value supplied");
```

Now if we for example don't specify a value we get the following error message (using `cargo run -- hello`):

![image-20210429184440864](C:\Users\gebruiker\AppData\Roaming\Typora\typora-user-images\image-20210429184440864.png)

Now we have to store the key and value to a file. So let's go to the [rust documentation](https://docs.rs/std). Here we see there's a library called FS that deals with filesystem manipulation including creating files so this is what we want to use. Here we can see there's a function write. The documentation here tells it all:

> This function will create a file if it does not exist, and will entirely replace its contents if it does.

Let's now write the key and the value to a file.

```rust
let contents: String = std::format!("{}\t{}\n",key,value);
let result: Result<(), Error> = std::fs::write("kv.db", contents);
```

You see that we use a new macro called format. The format macro will format a string and return it but not print in to standard out like print! does.

We see that we have a warning in our console if we use `cargo check`.

![image-20210429185541391](C:\Users\gebruiker\AppData\Roaming\Typora\typora-user-images\image-20210429185541391.png)

In normal programming we will throw exceptions and handle them however Rust tries to warn you on any potential crashes that might occur and expects you to handle them accordingly. Our write function uses a Result which has a possible error. Rust is asking us to handle that error. A result type either completes successful or it returns an error.

If we look closely at the signature of the Result object we see there's two extra parameters given to it ` <(),Error>` The `()` is what we call a unit or an empty tuple. It's used to indicate a void return type like we're used with other languages.

We can use pattern matching to handle the Result.  This is more or less a better switch statement.

```` rust
   match result {
        Ok(()) => {
            println!("Everything ran successfully");
        },
        Err(e)=> {
            eprintln!("{}",e.to_string());
        }
    }
````

So this is how we would formally handle a result object. However we can again just be brief and use an expect statement and let the program panic when it encounters the Error from the result object.

``` rust
std::fs::write("kv.db", contents).expect("Something went wrong with writing to a file");
```

Rust is not an object oriented language. But instead uses the concepts of structs which are basically structures with named elements.  

```rust
struct Database{}
```

So we want to be able to create this structure and add methods to our database Struct. This is done trough an implementation. To recap the struct field is used to add fields to our structures. and define the signature. The implementation block is used to add functionality to our structure.

```rust
impl Database{}
```

So we can use this implementation to make a constructor function called new.

```rust
impl Database{
	fn new() -> Database {
		Database {}
	}
} 
```

Now we can call this function with

```rust
let database: Database = Database::new()
```

Notice the function return type is specified with ***-> return type***. Please note that there is no default constructors as their are no objects. We basically implemented a function that does the same principle but it's not technically a constructor.

We want to add values and keys to our Database we can use a hashmap to store key value pairs so let's add it to our structures. This is the equivalent of a Dictionary in C#.

```rust
struct Database{
    map : std::collections::HashMap<String,String>
}
```

We also have to fill in this field in our new function:

````rust
impl Database {
	fn new() -> Database {
		Database {
			map: std::collections::HasMap::new()
		};
	}
}
````

However when our database is loaded in we want it to parse our database file and read in all the values we already stored to it and put those in our hashmap.

So first of to read a text file we can refer back to our documentation we see that there's also a read function that returns a result and gives back a vector of unsigned int. Because this is not very  practical we can use the read_to_string version. Let's also handle the errors correctly and use pattern matching to catch the errors:

```rust
    fn new() -> Result<Database, Error> {
        let contents= match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error);
            }
        };
        return Result::Ok(Database {
            map: HashMap::new()
        });
    }
```

This seems quite strange since you're binding the result of a match expression to a variable c but this is in fact possible in rust.  Also a little note on return, return is not necessary if it's the last statement.

This pattern is so common in rust that there's actually a shorthand for this:

``let contents = std::fs::read_to_string("kv.db")?`

So if the error is encountered it's thrown up if there's no error then we get our contents to our variable..

We also have to change our function call to new to reflect our result return type of the function

`let database: Database = Database::new().expect("Database::new() crashed");`

If kv.db is not there it would crash due to the expect being there however we will assume that it's there. Result is nothing more then an enum that returns something. Unlike other languages enum has data associated to it. 

Now we want to parse our string every key-value pair is seperated by new lines. we can ask for an iterator of each lines. We will loop over these lines using a foreach loop. Now we will split the line once by using line.split_once and specifying the character namely our tab that we enter. This returns an option of a tuple to us. The option we've already seen and basically denotes the possibility of null (since rust doesn't use null but rather None). So we use expect here so if our parsing failed we can see that the database is corrupted. A tuple is an array that only contains a fixed length of elements. They also allow multiple items from different types. We can also deconstruct it in our line itself. 

```rust
for line in contents.lines(){
	let (key,value) = line.split_once('\t').expect("database is corrupt");
}
```

However we will get an exception here denoting that the split_once function is experimental and therefor might not be stable to use. We can opt in to using unstable functions if we want to but for this example lets just change it to splitn which basically does the same thing:

```rust
        for line in contents.lines(){
            let mut chunks =line.splitn(2,'\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
        }
```

Now we want to be able to put stuff in our map so we have to declare the map above our parsing function:

```rust
let mut map = std::collections::HashMap::new()	
```

Now we insert the key and value inside of our for loop:

`map.insert(key,value);`

but if you see we'll get a compilation error. Our next on our chunk line returns not a string but the type `&str` this is due to the borrowing principle. Rust **doesn't have a garbage collector**. Instead it has the principle of ownership. Each chunk of memory has an owner so for example if we say `let test: String = "hello world"` then the variable test is the owner of the string "Hello world".

As soon as our variable goes our of scope (aka the function ended and the variable is gone from use) it frees up the memory associated with that variable. Let's look at this principle a little closer:

```rust
    fn new() -> Result<Database, Error> {
        /*let contents= match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error);
            }
        };*/
        let mut map = HashMap::new();
        //read the kv.db file
        let contents = std::fs::read_to_string("kv.db")?;
        //parse the string
        for line in contents.lines(){
            let mut chunks =line.splitn(2,'\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            //populate the hashmap
            map.insert(key,value);
        }

        return Result::Ok(Database {
            map:map
        });
    }
```

You would expect our variable map at line 8 to go out of scope and free up the memory at the end of our new function. This however doesn't happen considering the Database object we're returning contains a reference to our map so therefor the memory doesn't get freed up.  So this database owns the map. So ownership is moved from the map variable to the database variable.

so now that we understand this principle we can look at this chunk of code more closely:

```rust
      let contents = std::fs::read_to_string("kv.db")?;
        //parse the string
        for line in contents.lines(){
            let mut chunks =line.splitn(2,'\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            //populate the hashmap
            map.insert(key,value);
        }
```

Our contents variable owns our string it reads the contents into memory and is the owner of this memory chunk. However if we look more closely at contents.lines() we can see it returns a &str type. This is a reference. Why? Well this iterator is technically only a pointer to the next memory address of a line. Because the contents of content is not transferred ownership to our iterator. Once contents is dropped we can never use line because it's only a pointer.

So because key and value are also just references to strings what we're doing is inserting references into our hashmap instead of owned values so we need to basically make an owned string out of these references.

So what we want to do is copy the data that the reference is looking at and copy it into new memory so our map is going to contain a new memory with a copy of the data. We do this by adding a function to_owned().

`map.insert(key.to_owned(),value.to_owned());`

String::from would also be a way of doing this.

