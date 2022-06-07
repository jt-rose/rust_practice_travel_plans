mod vectors;

use vectors::*;

fn main() {
    // init travel plans
    let mut travel_plans = TravelPlans::new();

    // add cities
    travel_plans.add_visited_city("Amsterdam".to_string());
    travel_plans.add_visited_city("Seoul".to_string());

    // print out visited cities
    for city in travel_plans.visited_cities.iter() {
        println!("{} has been visited already", city);
    }

    // add to wish list, and then reject if already on list or already visited
    travel_plans.add_to_wish_list("Brazil".to_string());
    travel_plans.add_to_wish_list("Brazil".to_string());
    travel_plans.add_to_wish_list("Seoul".to_string());

    // print traveller description
    println!("{}", travel_plans.get_traveller_description());
}
