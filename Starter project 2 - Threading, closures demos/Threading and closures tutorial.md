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

