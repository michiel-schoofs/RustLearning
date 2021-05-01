# Rust Learning Project - Key value dictionary Part 2

Something we didn't touch on is why we're using &str as opposed to String. This is because they're internally something completely different String consists of the following elements

- Pointer to the data on the heap.
- The length of the data itself 
- The capacity of the String or how many data is allocated on the heap for this string. So how much data do we have reserved.

The &str doesn't contain the Capacity element. Because we're not gonna add to the String. You can't add to the &str. It's only a pointer and how long the data itself is so we can read. in conclusion is String read-write and &str read only.

The database program we've written has a bug. Every time we write to our database the whole entire file is overwritten. So that's the issue we're going to tackle now. We want to add a method to our database to insert key values. Now we've already seen how to make functions, but how de we add methods. Well a method is nothing more or less then a function where the first argument is a reference to the structure itself.

```rust
fn insert(self,key: String, value:String) -> () {}
```

This allows us to the following syntax

```rust
let database: Database = Database::new().expect("Database::new() crashed");
database.insert(key,value);
```

Before we can add data to our map you have to remember everything by default is immutable in Rust.  So we can't add data to anything there for we need to specify that our self variable is mutable and that we can change stuff in it

```rust
fn insert(mut self, key:String, value:String){
	self.map.insert(key,value);
}
```

there's also an extra functionality we want to use mainly that we also want to store our Key uppercased as well so if we associate hello to world we also want the key HELLO to refer to our value world. How do we do this?

We have a function called to uppercase if we look at the documentation we can see the self parameter being the first and only parameter of this function so we see it's a method and we can use it as this

```rust
database.insert(key.to_uppercase(), value);
```

We see that there's a lot of errors. If we try calling this while it would work in most languages. One of the first errors is that well because we already called the insert function the database is put in the variable self and passed to our function insert. If we remember after a function call unless we explicitly return it the self variable is freed from memory so the space occupied by our database structure gets freed up. There's also another issue here that the key and the value has the same issue as our database. The memory is freed up after our function call. But if we move places of these two functions our error of key disappears why is that?

```rust
    database.insert(key.to_uppercase(), value);
    database.insert(key,value);
```

Well if we look very closely at our to_uppercase function we see that it borrows the string key. what does this mean well it takes in a reference to string and returns a completely new string. It also returns the original string itself therefor it doesn't deallocate the memory of the original string while also returning a new string in memory therefor key is still available for use. This is called **borrowing**.

To reference memory or aka to borrow a value we add an & to it. So we borrow it. So with this we can fix the issue with changing the function signature:

```rust
fn insert(&mut self, key: String, value: String){
...
}
```

Now we have another error namely that our original variable database is not mutable. So we fix this by changing the database variable

```rust
let mut database: Database = Database::new().expect("Database::new() crashed");
```

We still have the question of value. Let's examine the string value, it gets assigned on the iterator line and then we move it into our insert function. We didn't pass it by reference so it should get freed at the end of the function but if we think about it we push it into our map of self. So it's not freed at the function. It's moved in the map. And the memory eventually gets freed up at the end of the main function. So it doesn't make sense to try and give that same memory a second time to the insert function. It's already moved and out of ownership in the main function. So how do we solve this dilemma?

Well we can use something called clone. if we look at the function signature we can immediately grasp what it does. It borrows the original value meaning that it will give back the ownership after the call and then it returns a new copy of the value. So this does solve our dilemma we're giving a copy of the value variable to our first call and giving the original to our second call.

```rust
database.insert(key.to_uppercase(), value.clone());
database.insert(key,value);
```

Now we have our basic database and it adds values however there's an issue. Whenever the program is shutdown so we need to add a method to print out the result to disk. We will call this method flush:

```rust
fn flush(&self) -> Result<(),std::io::Error> {
	let contents = String::new();
    for pairs : (String,String) in self.map {
        
    }
    todo!("Still busy writing this");
}
```

We already know the result type so if this operation completed successfully then we return a None type or unit type and if it fails we'll return an IO error. We see that if we iterate over a map we get the key value pairs back. The pairs are a tuple (Different types at each index and fixed length array). You can get rid of compiler errors by adding a todo macro. It will still panic when reaching it but it's a nice way of getting rid of those errors temporarily. We address the elements of a tupple by adding a '.' With the index (it's zero indexed).

We can now complete our code:

```rust
let mut contents:String = String::new();
for pairs in self.map{
	let line : String = format!("{}\t{}\n",pairs.0,pairs.1);
	contents.push_str(&line);
}
```

We first make our contents mutable we're going to edit the string itself. Next up we use a function called push_str to append a string to the contents itself. This takes a reference so it doesn't free the memory after the function call or transfer ownership. We can still use the line variable after our function call.

Now we can write this to disk. this already returns a Result<(),std::io::Error> so we can just use this as our return:

```` rust
    fn flush(self) -> Result<(), std::io::Error>{
        let mut contents:String = String::new();
        for pairs in self.map{
            let line : String = format!("{}\t{}\n",pairs.0,pairs.1);
            contents.push_str(&line);
        }

        return std::fs::write("kv.db", contents);
    }
````

Now lets just use expect on our flush method so it crashes in case of error and we have  a working database.

`database.flush().expect("Something went wrong writing the database to disk");`

A final thing to not about this flush function is because we've taken ownership this is the last method that can be called on our database. This is because we take ownership over self and don't return it. This is the behaviour we want because after flush is called we don't want people to be able to call other methods.

We can do a bit of optimization in our flush method:

```rust
    fn flush(self) -> Result<(), std::io::Error>{
        let mut contents:String = String::new();
        for (key,value) in &self.map{
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        return std::fs::write("kv.db", contents);
    }
```

First of we dereference the pair in line to make it easier to read, next up we can also borrow the self.map variable. instead of borrowing key and value separately. The different push and push_str function calls avoid us having to allocate a new variable to hold our line.

Just like in other languages you can implement certain interfaces. Drop is one of those **traits** that gets called when a specific struct goes out of scope so basically we can call our flush method here and whenever we're done using our database it automatically flushes to memory. This is the principle behind a so called deconstructor.

```rust
impl Drop for Database{
    fn drop (&mut self){
        let mut contents:String = String::new();
        for (key,value) in &self.map{
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents);
    }
}
```

But now we have our code on two places so the anwser is moving this function out of our database and drop method so we can call it from both places. So now we have the following structure:

```rust
impl Database{
	...
	
	fn flush(self) -> Result<(), std::io::Error>{
        return flush(&self);
    }
}

impl Drop for Database{
    fn drop (&mut self){
        let _ = flush(self);
    }
}

fn flush(database: &Database) -> Result<(), std::io::Error>{
    let mut contents:String = String::new();
    for (key,value) in &database.map{
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    return std::fs::write("kv.db", contents);
}
```

The problem is if we call flush manually it gets called again in the deconstructor. So we can simply introduce a boolean to handle this.

We add it to our structure:

```rust
struct Database {
    map: HashMap<String, String>,
    flush: bool
}

```

At the end of our new function we set flush to false:

```rust
return Result::Ok(Database {
    map,
    flush: false
});
```

And when flush is called set flush to true.

```rust
    fn flush(mut self) -> Result<(), std::io::Error>{
        self.flush = true;
        return flush(&self);
    }
```

Then in our deconstructor call we can simply check if flush is false and only then call flush.

```` rust
impl Drop for Database{
    fn drop (&mut self){
        if !self.flush {
            let _ = flush(self);
        }
    }
}
````

