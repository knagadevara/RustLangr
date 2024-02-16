/*
 Static variables:
 1. Data-type of the variable should be explicitly provided.
 2. Identifier should be in all caps.
 3. For immutable variables initialization happens ONLY ONCE.
    at compiletime value is evaluated and memory is directly allocated,
    for runtime using a wrapper delayed evaluation of value will take place.

   Cannot initialize a run-time value directly, can only be done using a wrapper.
   static TIME_STAMP_END: DateTime<Utc>  = Utc::now(); // dosent work

 4. They are available to all the functions defined in the file
    and not destroyed till the end of program runtime.
 6. They can also be imported into different modules available in same crate.

 Difference between a const and static
 Const would not be allocated memory, it will be replaced by its assigned value
 in the code at all the declarations.
*/

use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

const _MYCONST_1: u8 = 1; // Acts as a place holder for 1.
                          // static variable_name <wrapper<return-data-type>> = <wrapper::<Type>::constructor()>
pub static TIME_DELAY_3: Lazy<DateTime<Utc>> = Lazy::<DateTime<Utc>>::new(|| {
    println!("Closure Initialized: TIME_DELAY_3");
    sleep(Duration::new(5, 0));
    let time_now = Utc::now();
    return time_now;
});

// Memory allocation for &str type variables is done at compiletime.
// It is not possible to determines size of the assignment value for
// GA_MG_1 of type &str as it is dependant on TIME_DELAY_3 which
// would be determined at runtime, hence it throws error.
// pub static GA_MG_1: &str = *TIME_DELAY_3.to_rfc2822();

pub static GA_MG_2: &str = "Summer's comming!";

fn whats_the_time() {
    println!("In Func-> whats_the_time");
    // introducing 2sec delay
    sleep(Duration::new(2, 0));
    // var cannot be used outside of function scope.
    // the contents will be same for its life time until function finishes.
    let time_now = Utc::now();
    println!("Non Static Variable +2sec delay -> {}", time_now);

    // Adding another 2 sec delay
    sleep(Duration::new(2, 0));
    println!("Fn-First Call ->  {}", *TIME_DELAY_3);
    println!("Fn-Second Call -> {}", *TIME_DELAY_3);
    println!("Fn-Printing Time -> {}", TIME_DELAY_3.time());
    println!("Fn-Printing Date -> {}", TIME_DELAY_3.date_naive());
    println!("Out Func-> whats_the_time");
}

pub fn do_it() {
    println!("First Call of TIME_DELAY_3 from Main ->  {}", *TIME_DELAY_3);
    whats_the_time();
    println!(
        "Second Call of TIME_DELAY_3 from Main ->  {}",
        *TIME_DELAY_3
    );
    println!("GA_MG_2: {}", GA_MG_2);
}
