
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

