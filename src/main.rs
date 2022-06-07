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

    let mut travel_plans = TravelPlans::new();
    travel_plans.add_visited_city("Amsterdam".to_string());
    travel_plans.add_visited_city("Seoul".to_string());
    for city in travel_plans.visited_cities.iter() {
        println!("{} has been visited already", city);
    }

    let traveller_description = match travel_plans.seasoned_traveller {
        true => "She has travelled a great deal",
        false => "she has not travelled so much"
    };

    println!("{}", traveller_description);
}
