// Rust equivalents to ES6 array methods

pub struct ArrayMethods {
    strings: Vec<String>,
    numbers: Vec<u8>
}

impl ArrayMethods {
    pub fn new() -> Self {
        Self{
            strings: vec!["Tokyo".to_string(), "Paris".to_string(), "London".to_string()],
            numbers: vec![1,2,3,4,5]
        }
    }

    // map => iter().map(|x| {}).collect()
    pub fn double(&self) -> Vec<u8> {
        self.numbers.iter().map(|num| {
            num * 2
        }).collect()
    }

    // filter => filter / retain
    pub fn filter(&self) -> Vec<&String> {
        self.strings.iter().filter(|s| {
            s == &"Paris"
        }).collect()
    }

    // find
    pub fn find_paris(&self) -> Option<&String> {
        self.strings.iter().find(|city| {
            city == &"Paris"
        })
    }

    // findIndex => position
    pub fn find_paris_index(&self) -> Option<usize> {
        self.strings.iter().position(|city| {
            city == &"Paris"
        })
    }

    // some => any
    pub fn find_some_city(&self, target_city: &str) -> bool {
        self.strings.iter().any(|city| {
            city == target_city
        })
    }

    // every => all
    pub fn all_less_than_10(&self) -> bool {
        let limit: u8 = 10;
        self.numbers.iter().all(|num| {
            num < &limit
        })
    }


    // reduce
    pub fn sum(&self) -> u8 {
        self.numbers.iter().sum()
    }

    // reduce alternative - fold
    pub fn get_char_count(&self) -> usize {
        self.strings.iter().fold(0, |acc, x| {
            acc + x.len()
        })
    }

}