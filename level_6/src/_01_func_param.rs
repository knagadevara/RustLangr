/*
Function Parameters can be of two types
Pass by value -> Immutable
    param_by_value(uint: u32, text: String)
Pass by reference -> Can be mutable
    pass_by_ref(uint: &u32, text: &mut String)

u32 implements 'COPY' trait which does a bit-copy, creating a new function scope variable.
String does not implement 'COPY' trait so the ownership is moved
and the original string in heap will not be availale afer function scope
as 'Drop' trait will de-allocate the memory

*/

fn param_by_value(uint: u32, text: String) {
    println!("param_by_value");
    println!("RlNo: {}", uint);
    println!("Name: {}", text);
}

fn pass_by_ref(uint: &u32, text: &mut String) {
    println!("pass_by_ref");
    println!("RlNo: {}", uint);
    println!("Name: {:#?}", text.make_ascii_uppercase()); // Changes the original contents of string
}

// immutable func param.
fn pass_string(word: &str) {
    let tmp_str = word.to_ascii_uppercase();
    println!("{}", tmp_str);
}

fn hrd_order_func() {
    let my_roll: [u32; 2] = [990, 560];
    let my_name: [&str; 2] = ["RAM", "Krishna"];
    let rno_2: u32 = 110;
    let mut name_2: String = String::from("Shiva");
    pass_by_ref(&rno_2, &mut name_2);
    println!(
        "Calling String Object after passing its reference\nStudent Name: {}",
        name_2
    );
    param_by_value(rno_2, name_2);
    param_by_value(my_roll[0], String::from(my_name[0]));
    println!("Call from HO Func");
    println!("Roll Number: {}", my_roll[1]);
    println!("Student Name: {}", my_name[1]);
    println!("Ownership of the object name_2 moved to parm-variable and it is lost after function scope. Calling String Object Student Name: Errors out as");
}

pub fn do_it() {
    param_by_value(560, String::from("Ram"));
    hrd_order_func();
    // It is not possible to pass a string directly to a function which accepts a String object/reference.
    // It would be a good option to use &str
    pass_string("Hello!!");
}
