// Crate-Level attribute will apply on all over the file.
#![allow(dead_code)]

pub enum Color {
    Grey,
    MetallicSilver,
    MidnightBlack,
    RoyalDeepBlue,
}
pub enum PowerTrain {
    NaturalAspired,
    Turbo,
    TurboGDI,
    MildHybrid,
    StrongHybrid,
    Electric,
}
pub enum Gender {
    Male,
    Female,
}

pub enum State {
    AndhraPradesh,
    Telangana,
    Tamilnadu,
    Karnataka,
    Kerala,
}

pub enum Bharath {
    EastBharath,
    WestBharath,
    CentralBharath,
    NorthBharath,
    SouthBharath,
}

// Creating Data Enums
pub enum AddressParameters {
    AddressLine(String),
    State(State),
    PinCode(u32),
    OfBharath(Bharath),
    Null,
}
