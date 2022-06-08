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
    pub fn findParis(&self) -> Option<&String> {
        self.strings.iter().find(|city| {
            city == &"Paris"
        })
    }

    // findIndex

    // some

    // every

    // reduce


}