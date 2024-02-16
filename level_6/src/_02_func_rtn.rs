/*
    Syntax:

    fn func_name() -> return_data_type/&return_data_type {
        .....
        .....
        return accepted_value_of_data_type;
    }

    // One line return function.
    fn func_x() -> return_data_type/&return_data_type {expression/variable}

    In Rust return statement is either the last statement without semi-colon
    or the one with 'return' keyword to it.

    if the returning type does not implement 'Copy' then ownership will be transfered.
    else bit-copy will happen without ownership transfer.
    returning u32 -> bit-copy happens, Ownership is not transfered.
    returning String -> complete Ownership is transfered.
*/

use std::intrinsics::mir::Len;

// returning a value
fn area_of_square(side: u32) -> u32 {
    side * side
}
fn area_of_rect(lt: f32, bth: f32) -> f32 {
    return lt * bth;
}

// Returning strings
fn chk_mrks_1(mks: u8) -> String {
    if mks < 35 {
        "FAIL".to_string() // converts a literal to object
    } else {
        "PASS".to_string()
    }
}
// Returning a literal
// static ensures the literal will stay until program lifetime.
fn chk_mrks_2(mks: u8) -> &'static str {
    if mks < 35 {
        "FAIL"
    } else {
        "PASS"
    }
}

fn get_first_word(words: &str) -> &str {
    let mut idx: usize = 0;
    for letter in words.chars() {
        if letter == ' ' {
            break;
        } else {
            idx += 1;
            continue;
        }
    }
    return &words[..idx];
}

// returning mutable reference
fn mut_stg(some_str: &mut str) -> &mut str {
    some_str.make_ascii_uppercase();
    return some_str;
}

/*
fn bad_rtn_1() -> &String {
    let my_name: String = String::from("Sriman");
    // memory for my_name will be de-allocated after the scope of function
    // hence it is not possible to return its reference.
    return &my_name; // Errors Out
}


fn bad_rtn_1(name: String) -> &String {
    // As function param is a string-value, ownership will be transfered to name variable.
    // memory for name will be de-allocated after the scope of function
    // hence it is not possible to return its reference.
    let name: String = String::from("Sriman");
    return &name; // Errors Out
}
*/
// Complex Option Return
fn is_true(age: u8) -> (u8, Option<bool>, Option<&'static str>) {
    if age >= 18 {
        return (age, Some(true), Some("Eligible"));
    } else {
        return (age, None, None);
    }
}

pub fn do_it() {
    println!("{}", area_of_square(10));
    println!("{}", area_of_rect(33.0, 58.0));
    let mut hm_add: String = String::from("Gach");
    println!("Val: {}  Add: {:p}", hm_add, hm_add.as_ptr());
    mut_stg(&mut hm_add);
    println!("Val: {}  Add: {:p}", hm_add, hm_add.as_ptr());
    let chk_vt = is_true(21);

    match chk_vt {
        (age, Some(t), Some(s)) => println!("Voter is {0} old. Hence {1}", age, s),
        (age, None, Some(_)) => println!("Not the right age to Vote"),
        (age, Some(_), None) => println!("Not the right age to Vote"),
        (age, None, None) => println!("Not the right age to Vote"),
    }

    println!("chk_mrks_1: {}", chk_mrks_1(20));
    println!("chk_mrks_2: {}", chk_mrks_2(120));
    println!("First Word: {}", get_first_word("Picollo King"));
}
