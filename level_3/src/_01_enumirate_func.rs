mod _01_types;
use _01_types::{Color,PowerTrain};
pub fn simple_enum(){

  let my_color: Color = Color::RoyalDeepBlue;

  match my_color {    
    Color::MidnightBlack => println!("Black"),
    Color::MetallicSilver => println!("Silver"),
    Color::RoyalDeepBlue => println!("Blue")
  }

}