#[cfg(test)]
mod read_int_and_print {

    #[test]
    fn run() {

        #[allow(unused_mut)]
        let mut temp = String::from("123\n");
        //stdin().read_line(& mut temp).expect("error when reading");
        let i:i32 = temp.trim().parse().unwrap();

        println!("{}",i)

    }
}