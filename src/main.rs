mod vectors;

use vectors::*;

fn main() {
    // init travel plans
    let mut travel_plans = TravelPlans::new();

    // add cities
    travel_plans.add_visited_city("Amsterdam");
    travel_plans.add_visited_city("Amsterdam");
    travel_plans.add_visited_city("Seoul");

    // add to wish list, and then reject if already on list or already visited
    travel_plans.add_to_wish_list("Brazil");
    travel_plans.add_to_wish_list("Brazil");
    travel_plans.add_to_wish_list("Seoul");

    // move city from wish list to visited
    travel_plans.add_visited_city("Brazil");
    travel_plans.add_to_wish_list("Brazil");

    // remove from wish list
    travel_plans.add_to_wish_list("Melbourne");
    travel_plans.remove_from_wish_list("Melbourne");

    // print out visited cities
    for city in travel_plans.visited_cities.iter() {
        println!("{} has been visited already", city);
    }

    // print out wish list
    for city in travel_plans.travel_wish_list.iter() {
        println!("{} is on your wish list", city);
    }

    // print traveller description
    println!("{}", travel_plans.get_traveller_description());
}
