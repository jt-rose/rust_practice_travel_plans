mod vectors;
mod es6;

use std::process::id;
use vectors::*;
use es6::*;

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
    travel_plans.print_visited_cities();

    // print out wish list
    travel_plans.print_wish_list();

    // print traveller description
    println!("{}", travel_plans.get_traveller_description());

    // test es6 methods

    let es = ArrayMethods::new();

    // test map
    for num in es.double().iter() {
        println!("{}", num);
    }

    // test filter
    for item in es.filter().iter() {
        println!("{}", item);
    }

    // test find Paris
    let found_paris = es.find_paris();

    if let Some(city) = found_paris {
        println!("We found {}", city);
    } else {
        println!("No city found!");
    }

    // test finding index of paris
    let found_paris_index = es.find_paris_index();

    if let Some(idx) = found_paris_index {
        println!("Paris index is {}", idx);
    } else {
        println!("No index found!");
    }

    let london_found = es.find_some_city("London");
    println!("London was found on the list of cities: {}", london_found);

    // test the all method
    let all_less_than_10 = es.all_less_than_10();
    println!("All numbers are less than 10: {}", all_less_than_10);

    // test sum method
    let sum = es.sum();
    println!("The sum of all the numbers is {}", sum);

    // test fold function
    println!("Total char count: {}", es.get_char_count());
}
