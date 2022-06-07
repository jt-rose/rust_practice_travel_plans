pub struct TravelPlans {
    pub visited_cities: Vec<String>,
    pub travel_wish_list: Vec<String>,
}

enum VisitStatus {
    Visited,
    OnWishList,
    NotListed
}

impl TravelPlans {
    pub fn new() -> Self {
        Self{
            visited_cities: vec![],
            travel_wish_list: vec![]
        }
    }

    fn check_already_listed(&self, city: &str) -> VisitStatus {
        for visited_city in self.visited_cities.iter() {
            if visited_city == city {
                return VisitStatus::Visited;
            }
        }

            for wish_list_item in self.travel_wish_list.iter() {
                if wish_list_item == city {
                    return VisitStatus::OnWishList;
                }
        }

        return VisitStatus::NotListed;
    }

    pub fn get_traveller_description(&self) -> &str {
        if self.visited_cities.len() > 8 {
            return "She has travelled a great deal"
        }
            return "she has not travelled so much"
    }

    pub fn add_visited_city(&mut self, city: &str) {
        let visit_status = self.check_already_listed(city);

        match visit_status {
            VisitStatus::NotListed => self.visited_cities.push(city.to_string()),
            VisitStatus::Visited => println!("You've already crossed off {}!", city), // why not printing?
            VisitStatus::OnWishList => {
                self.travel_wish_list.retain_mut(|wish| {
                    wish != city
                });
                self.visited_cities.push(city.to_string());
            }
        }
    }

    pub fn remove_visited_city(&mut self, city: String) {
        self.visited_cities.retain_mut(|visited_city| {
            visited_city != &city
        })
    }

    pub fn  add_to_wish_list(&mut self, city: &str) {
        let visit_status = self.check_already_listed(city);

        match visit_status {
            VisitStatus::Visited => println!("You've already been to {}!", city),
            VisitStatus::OnWishList => println!("{} is already on your wish list!", city),
            VisitStatus::NotListed => {
                println!("Can't wait to visit {}!", city);
                self.travel_wish_list.push(city.to_string());
            }
        }
    }

    pub fn remove_from_wish_list(&mut self, city: &str) {
        self.travel_wish_list.retain_mut(|wish| {
            wish != city
        })
    }

    pub fn print_visited_cities(&self) {

        if self.visited_cities.len() <= 0 {
            println!("No cities visited yet");
        } else {
            println!("Visited Cities:");
            for city in self.visited_cities.iter() {
                println!("  {}", city);
            }
        }
    }

    pub fn print_wish_list(&self) {

        if self.travel_wish_list.len() <= 0 {
            println!("No cities on wish list yet")
        } else {
            println!("Wish List:");
            for city in self.travel_wish_list.iter() {
                println!("  {}", city);
            }
        }
    }
}