mod bollywood;
mod hollywood;
use crate::hollywood::submod2::child1::sub_mode_hole_again;

fn main() {
    println!(
        "Inline module...............
    ......................."
    );
    //  calling inline module
    message::say_my_name();
    //  calling nested inline
    marvel::marvel_collections();
    marvel::spider_man::spider_man_collections();
    println!("End of nested Inline module");
    println!("....................................");
    println!("Starting of the external modules system");
    // There are two ways to organize our code using external modules in Rust, and we can mix and match both ways
    // The first option is file based module:- It allows us to create modules in files with matching names.
    bollywood::tseries();
    hollywood::future_se_hai_apun();

    // submodules
    bollywood::bollsubmod::boll_sub_mod();
    hollywood::submod1::warner_brother();
    hollywood::submod2::sub_mod_hole();
    hollywood::submod1::calling_parent();
    hollywood::submod1::calling_again();

    // ,...................................
   sub_mode_hole_again();

}

// inline module
mod message {
    pub fn say_my_name() {
        println!("Hi my name is inline IRON-MAN!")
    }
}

// nested inline
mod marvel {
    pub mod spider_man {
        pub fn spider_man_collections() {
            println!("Spider man home coming!");
        }
    }
    pub fn marvel_collections() {
        println!("Iron man series!")
    }
}

// all the above is inline module

// Now we will explore external module, beacuse this is the one which we use frequent in our projects
// Let's go..........................................
