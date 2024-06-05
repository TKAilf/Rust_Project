use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // クロージャ：外部の環境を使用する
    // let x = 4;
    // let equal_to_x = |z| z==x;
    // let y = 4;
    // assert!(equal_to_x(y));
    // クロージャにはFn, FnOnce, FnMutの3つがある。
}

// fn simulated_expensive_calculation(intensity: u32) -> u32{
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25{
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }else{
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T, K, V>
    where T: Fn(K) -> V
{
    calculation: T,
    values: HashMap<K, V>,
}

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher{
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32{
//         match self.value{
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// ハッシュマップを使用し、引数にジェネリック型を使用するように改変する
impl<T, K, V> Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Clone,
          V: Clone
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.values.insert(arg, v.clone());
                v
            }
        }
    }
}
