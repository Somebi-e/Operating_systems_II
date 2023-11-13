///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;
//mod how_you_hold_data_for_operations::primatives;
//mod derived;

extern crate hello_world_lib;

///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line

use greetings::{english, spanish, french} ;
//use how_you_hold_data_for_operations::primatives::compound_type;

fn main() {

    let mut _greeting: &str ="Hello There";
    _greeting = "Hello there again";
    //all primitive data types are located on the stack
    //string with S is located on the heap
    let a_float :f64 = 4.5;
    println!("{}", a_float);

    println!("Hello, world!");
    println!("{}", english::default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());
    println!("from Scalar_types, {}", how_you_hold_data_for_operations::primatives::compound_type::multipler(&a_float));
    how_you_hold_data_for_operations::derived::user_defined::run();


}

