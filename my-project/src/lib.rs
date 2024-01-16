#!/usr/bin/rustc

mod front_of_house {
    mod hosting() {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // using absolute paths
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();

    //using relative path
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
}