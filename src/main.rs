mod vectors;

fn main() {
    println!("Hello, world!");
    vectors::print_something();
    let mut cities = vectors::get_cities();
    println!("First city is {}", cities[0]);

    for city in vectors::filter_visited_cities(&vectors::get_cities()).iter() {
        println!("{} has not been visited", city);
    }
}
