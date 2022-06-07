mod vectors;

use vectors::*;

fn main() {
    println!("Hello, world!");
    print_something();
    let mut cities = get_cities();
    println!("First city is {}", cities[0]);

    for city in filter_visited_cities(&get_cities()).iter() {
        println!("{} has not been visited", city);
    }
}
