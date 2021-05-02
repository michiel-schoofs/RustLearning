# Threading and closures tutorial

We're going to build a library to make our own threadpooling. This is just to get familiar with the concept of libraries and threadings and also testing. The first thing is obviously to start a new project using cargo. this time we're gonna write a library so we use the command `cargo new --lib threadpool` .

So let's first define our global sturcture. We need a public structure so other applications can interface with our structure but we don't know what we have to write in there just yet. Let's also provide an implementation for this structure as we've seen in our first project. Let's also make a new function so we can make an instance of our Thread pool and lets also make an execute function so we can actually execute the tasks in our thread pool. 

So in code this looks like this:

```rust
pub struct ThreadPool{}

impl ThreadPool{
    fn new() -> Self {
        Self
    }
    
    fn execute(&self) -> (){}
}
```

Not that unlike the previous tutorial we don't need to specify the name of our structure in our 'constructor' function. We can just reference this structure by using Self. Note that we use a borrowing of self so we don't give ownership to execute because we want to be able to call execute over and over and we want to use our thread pool after execute is called on it.

Now let's write a test:

```rust
#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn it_works() {
        let threadPool:ThreadPool = ThreadPool::new();
        threadPool.execute();
    }
}
```

## Closures

Now we actually want to be able to pass a function to our execute method to do this we use a principle called closures. Closures are in essence anonymous functions you can save in a variable or pass as an argument to other functions. I'll explain this concepts with some code taken from the documentations. You can find the source of this in a separate project 'closures_demo'.

The demonstrate closures we will use the following  hypothetical scenario:

-  There's a function that will take a long time to compute called expensive_caluclation we mock this function with the following code:

```rust
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

- We also add a main function that will take two numbers as an input and call a function on those. We take the intensity and a so called random number (we just hardcode) and execute a function. The code for our main function is the following:

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

- Now let's make a function like this:

  ```rust
  fn generate_workout(intensity: u32, random_number: u32) {
      if intensity < 25 {
          println!(
              "Today, do {} pushups!",
              simulated_expensive_calculation(intensity)
          );
          println!(
              "Next, do {} situps!",
              simulated_expensive_calculation(intensity)
          );
      } else {
          if random_number == 3 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!(
                  "Today, run for {} minutes!",
                  simulated_expensive_calculation(intensity)
              );
          }
      }
  }
  ```

We see that there's a lot of calls in this function to our expensive function simulated_expensive_calculation. In the first if block it's even called twice.

The first solution would be to move the function call to a variable and save the first if statement on executing multiple times the same function:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let calculated = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!",calculated);
        println!("Next, do {} situps!",calculated);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",calculated);
        }
    }
}
```

The disadvantage to this approach is that in the if block inside of the else block we don't need the calculated function at all considering it takes some time we're always waiting on it even if it's not necessary Hench slowing down our application.

So what we can do is basically let the variable calculated hold the function itself and we only execute it whenever we need it. This is called a closure and the syntax is the following: Start with introducing the variable then after the equals sign specify the parameters within ||. This is similar to ruby and small talks. Next up we can execute the function using a normal function call:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let calculated = |num : u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    };

    if intensity < 25 {
        println!("Today, do {} pushups!",calculated(intensity));
        println!("Next, do {} situps!",calculated(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",calculated(intensity));
        }
    }
}
```

This solves the issue of our else if block but reintroduces the problem with the double if block. Closures have a solution for this however.

Also note that you can omit a lot of optional syntax from the closure itself note that all of these are equivalent:

````rust
let add_1 = (x: u8) -> u8 {x + 1}
let add_2 = |x: u8| {x+1};
let add_3 = |x| {x+1};
let add_4 = |x| x+1;
````

If we don't infer the type then it's possible to get an error. Namely if you call the same function with a string followed by an integer or call an illegal function on the inferred type within the enclosure. So this is quite unsafe code Hench why I prefer to specify everything.

To solve the issue we can define a new structure called cacher I'm giving the implementation here and then providing an explanation for them:

```rust
struct Cache<T> where T: Fn(u32) -> u32{
    func: T,
    result: Option<u32>
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T>{
        Cache{
            func: calculation,
            result: None
        }
    }

    fn value(&mut self,arg: u32) -> u32{
        match self.result{
            Some(v) => v,
            None => {
                let v = (self.func)(arg);
                self.result = Some(v);
                v
            }
        }
    }
}
```

First we make a generic structure that takes a closure as it's generic parameter. The closure itself implements something called a trait. A function has a trait of Fn  so we can use this to infer that we take in a closure in our generic parameter T. The next field result is an Option<u32>. Because when the structure is just made it's not gonna have a result so None and after it's called once it will contain a result. This is the definition of an Option it's either None or a specific value wrapped in Some.

We provide an implementation of our structure, and provide it with a new function so we can make a new instance of our Cache object. This takes in a closure. And initializes it with none since we have never run our function yet.

When value is called for the first time we provide an argument every subsequent call to it will just return our value we do this  with a match expression. This makes sure that we only call the closure once.

We can now rework our calculation code as such:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut calculated = Cache::new(|num : u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        println!("Today, do {} pushups!",calculated.value(intensity));
        println!("Next, do {} situps!",calculated.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",calculated.value(intensity));
        }
    }
}
```

This has a couple of limitations the first limitation this approach has is that it just accepts the first parameter for intensity so if we use different intesities this will just return the cached value.

This implementation is the following:

```rust
impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T>{
        Cache{
            func: calculation,
            result: HashMap::new()
        }
    }

    fn value(&mut self,arg: u32) -> u32{
        return match self.result.contains_key(&arg) {
            true => {
                self.result.get(&arg).expect("Something went wrong").to_owned()
            }
            false => {
                let v = (self.func)(arg);
                self.result.insert(arg, v);
                v
            }
        }
    }
}
```

We can also make our 

