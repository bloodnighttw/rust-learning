#[derive(Debug,Clone)]
struct ForTest {
    len:i32,
    str:String
}

#[cfg(test)]
mod _struct {
    use crate::var::strust::ForTest;

    #[test]
    fn run() {

        let var_with_ownership = ForTest {
            len: 0,
            str: String::new()
        }; // ownership because

        let take_ownership = var_with_ownership;
        let make_clone = take_ownership.clone(); // deep copy
        //println!("{} {}", var_with_ownership.len, take_ownership.str); //not allow
        println!("{} {} ",take_ownership.len,take_ownership.str);
        println!("{} {} ",make_clone.len,make_clone.str);


    }
}