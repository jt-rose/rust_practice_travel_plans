pub struct TravelPlans {
    pub visited_cities: Vec<String>,
    pub travel_wish_list: Vec<String>,
    pub seasoned_traveller: bool
}

impl TravelPlans {
    pub fn new() -> Self {
        Self{
            visited_cities: vec![],
            travel_wish_list: vec![],
            seasoned_traveller: false
        }
    }

    fn set_seasoned_traveller(&mut self) {
        if self.visited_cities.len() > 8 {
            self.seasoned_traveller = true;
        }
    }

    pub fn add_visited_city(&mut self, city: String) {
        self.visited_cities.push(city);
        self.set_seasoned_traveller();
    }

    pub fn remove_visited_city(&mut self, city: String) {
        self.visited_cities.retain_mut(|visited_city| {
            visited_city != &city
        })
    }

    pub fn  add_to_wish_list(&mut self, city: String) {
        let mut already_listed = false;

        for visited_city in self.visited_cities.iter() {
            if visited_city == &city {
                println!("You've already been to {}!", city);
                already_listed = true;
            }
        }

        if !already_listed {
            for new_city in self.travel_wish_list.iter() {
                if new_city == &city {
                    println!("{} is already on your wish list!", city);
                    already_listed = true;
                }
            }
        }

        if !already_listed {
            println!("Can't wait to visit {}!", city);
            self.travel_wish_list.push(city);
        }
    }
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

