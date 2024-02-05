use crate::_01_types::AddressParameters;
use crate::_01_types::Color;

pub fn simple_enum() {
    let my_color: Color = Color::RoyalDeepBlue;

    match my_color {
        Color::MidnightBlack => println!("Black"),
        Color::MetallicSilver => println!("Silver"),
        Color::RoyalDeepBlue => println!("Blue"),
        Color::Grey => println!("Grey!"),
    }
}

pub fn data_enum() {
    // let address_line_1 = AddressParameters::AddressLine("");
    // let address_line_2 = AddressParameters::AddressLine("");
    let state_of_add = AddressParameters::State(crate::_01_types::State::AndhraPradesh);
    let state_pincode = AddressParameters::PinCode(788123);
    let state_belongs = AddressParameters::OfBharath(crate::_01_types::Bharath::SouthBharath);

    match state_belongs {
        AddressParameters::OfBharath(_stg) => println!("Part of India!"),
        _ => println!("Noting Mate!"),
    }

    match state_pincode {
        AddressParameters::PinCode(pin) => println!("{}", pin),
        _ => println!("Noting Mate!"),
    }
    match state_of_add {
        AddressParameters::State(_st) => println!("State in India!"),
        _ => println!("Noting Mate!"),
    }

    print!(
        "Size of AddressParameter: {} bytes\n",
        std::mem::size_of::<AddressParameters>()
    )
}

pub fn get_in_sec(hour: u32, min: u32, sec: u32) -> Option<u32> {
    if hour < 23 && min < 59 && sec < 60 {
        let second_in_day: u32 = (hour * 3600) + (min * 60) + sec;
        return Option::Some(second_in_day);
    } else {
        return Option::None;
    }
}

pub fn get_contents_of_option() {
    let seconds: Option<u32> = get_in_sec(3, 45, 32);

    match seconds {
        Option::Some(s) => println!("Using Match {0}", s),
        Option::None => println!("No Return"),
    }

    // THere are otherways to get the data from OPtion. check https://doc.rust-lang.org/stable/core/option/enum.Option.html

    println!("Unwrapping {}", seconds.unwrap_or(0))
}

pub fn useing_result_enum() {
    // creating a variable which takes in type of result with a specific error in it.
    let dem_result: Result<i32, std::num::ParseIntError>;
    // Feeding a function whcih returns a Result<T,E>
    dem_result = i32::from_str_radix("FF", 16); // works well

    match dem_result {
        Result::Ok(ok) => println!("Result returned successfully {ok:?}"),
        Result::Err(err) => println!("Failed to Parse! {err:?}"),
    }

    // or can also be done through the listed function in Result enum
    // https://doc.rust-lang.org/stable/core/result/index.html

    let dem_result2 = i32::from_str_radix("VNSK", 11); // Fails well
    println!("{}", dem_result2.unwrap_or(-1));
}
