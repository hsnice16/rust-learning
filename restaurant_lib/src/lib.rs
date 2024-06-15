mod back_of_house;
mod front_of_house;

use crate::front_of_house::hosting;
// re-exporting
pub use crate::front_of_house::hosting as Hosting;

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist()
    }
}
