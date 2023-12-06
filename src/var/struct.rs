#[derive(Debug,Clone)]
struct ForTest {
    len:i32,
    str:String
}

trait Show{
    fn show(&self);
}

impl Show for ForTest {
    fn show(&self) {
        println!("{} {}",self.len,self.str);
    }
}

#[cfg(test)]
mod _struct {
    use crate::var::r#struct::{ForTest, Show};

    #[test]
    fn run() {

        let var_with_ownership = ForTest {
            len: 0,
            str: String::from("yo wtf")
        }; // ownership because

        let take_ownership = var_with_ownership;
        let make_clone = take_ownership.clone(); // deep copy
        //println!("{} {}", var_with_ownership.len, take_ownership.str); //not allow
        println!("{} {} ",take_ownership.len,take_ownership.str);
        println!("{} {} ",make_clone.len,make_clone.str);
        make_clone.show();


    }
}