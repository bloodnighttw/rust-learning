use std::fmt::{Display};

struct Weight<T>{
    size:T, // i8 i32 d32 d64 ...... etc
    measurement:String
}

struct Animal<T>{
    name:String,
    sound:String,
    pound: Weight<T>,
}

trait Sound{
    fn said(&self);
}

impl<T> Sound for Animal<T> where T:Display{
    fn said(&self) {
        println!("The {} is {} {} and it sound like \'{}\'",self.name,self.pound.size,self.pound.measurement,self.sound)
    }
}

#[cfg(test)]
mod generics {
    use crate::var::generics::{Animal, Sound, Weight};

    #[test]
    fn run() {
        let elephant = Animal{
            name:String::from("elephant"),
            sound: "trumpet".to_string(),
            pound: Weight {
                size: 3000,
                measurement: "kg".to_string(),
            },
        };

        let rat = Animal{
            name:"rat".to_string(),
            sound:"gi".to_string(),
            pound:Weight{
                size: 10.5,
                measurement: "g".to_string(),
            }
        };

        elephant.said();
        rat.said();
        
    }
}