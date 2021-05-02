use std::thread;
use std::time::Duration;
use std::collections::HashMap;


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cache<T> where T: Fn(Y) -> Z{
    func: T,
    result: HashMap<u32,u32>
}

impl<T> Cache<T> where T: Fn(u32) -> u32 ,  {
    fn new(calculation: T) -> Cache<T>{
        Cache{
            func: calculation,
            result: HashMap::new()
        }
    }

    fn value(&mut self,arg: Y) -> u32{
        return match self.result.contains_key(&arg) {
            true => {
                self.result.get(&arg).expect("Something went wrong").to_owned()
            }
            false => {
                let v = (self.func)(arg);
                self.result.insert(arg, v.clone());
                v
            }
        }
    }
}

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