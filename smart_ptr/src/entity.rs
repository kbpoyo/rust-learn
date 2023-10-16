use std::fmt;

pub trait Getter {
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}

pub struct Entity {
    name: String,
    age: u32,
    brif: String,
}

impl Entity {
    pub fn new(name: String, age: u32, brif: String) -> Self {
        Entity{name, age, brif}
    }

    pub fn get_brif(&self) -> &str {
        &self.brif
    }

}

impl Getter for Entity {
    
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
        
}

impl fmt::Display for Entity{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my name is {} and age is {}, this is my brif: {}", self.name, self.age, self.brif)
    }
}


#[derive(Debug)]
pub struct Animal {
    name: String,
    age: u32,
}

impl Animal {
    pub fn new(name: String, age: u32) -> Self{
        Animal { name, age}
    }
}

impl Getter for Animal {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_age(&self) -> u32 {
        self.age
    }
}

#[derive(Debug)]
pub struct NoOwnerShip {
    pub real_part: i32,
    pub vir_part: i32,
}

