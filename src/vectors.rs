
// pub can be accessed externally after importing the mod
pub fn print_something() {
    println!("something");
    cant_find_me();
}

// without pub, this function is hidden, even after importing this module
// but it can be used internally, as above
fn cant_find_me() {
    println!("I'm hidden to outside modules")
}

pub fn get_cities() -> Vec<String> {
    let cities = vec![
        "Tokyo".to_string(),
        "Paris".to_string(),
        "Berlin".to_string()
    ];
    cities
}

pub fn filter_visited_cities(cities: &Vec<String>) -> Vec<&String> {
    // list of visited cities
    let visited_cities = ["Tokyo".to_string(), "Paris".to_string()];
    let mut unvisited: Vec<&String> = vec![];

    // loop through city and visited list and store unvisited in above vector
    for city in cities.iter() {
        let mut already_visited = false;
        for visited_city in visited_cities.iter() {
            if city == visited_city {
                already_visited = true;
            }
        }
        if !already_visited {
            unvisited.push(city);
        }
    }

    // return unvisited list
    unvisited
}

