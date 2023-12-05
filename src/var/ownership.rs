#[cfg(test)]
mod ownership {

    #[test]
    fn stack() {

        // don't need ownership
        let store_on_stack = "this &str is store on stack";
        // don't take store_on_stack ownership because store_on_stack is store on stack
        let _copy_by_value = store_on_stack;
    }

    #[test]
    fn heap(){
        // store on heap
        let store_on_heap = String::from("store on heap");
        // or "store on heap".to_string()

        let take_ownership = store_on_heap;

        //println!("{}",store_on_heap);  not allow, ownership have been taken by take_ownership.
        println!("{}",take_ownership)
    }


}