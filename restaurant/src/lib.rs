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
}
use crate::front_of_house::hosting;

mod customer {
pub fn eat_at_restaurant()
{
   super::hosting::add_to_waitlist();
    super::hosting::seat_at_table();
    super::front_of_house::hosting::add_to_waitlist();
    super::front_of_house::hosting::seat_at_table();
}
}
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}