#!/usr/bin/rustc

mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

   pub mod serve_time {
       pub fn order_service() {}
        pub fn time_service() {}
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

pub fn payment_mode()
{
    crate::front_of_house::serve_time::order_service(); 
    crate::front_of_house::serve_time::time_service();
}

pub fn order_service_time () 
{
    crate::front_of_house::serve_time::order_service();
    crate::front_of_house::serve_time::time_service();

}