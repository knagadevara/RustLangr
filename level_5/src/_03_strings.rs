/*
   Strings can be created in two ways in Rust
   1. Immutable types '&str' -> string slice
       - Memory is statically allocated -> meaning it would stay till end of program.
       - It is basically a "fat pointer" to text enclosed in double quotes,
       i.e it contains address to first byte, and size (count of characters in D'quotes).
       ex: let _word = "Hello" // [address to first character 'H' , Size of complete Literal]
   2. Structure 'String'
       - Mutable type. Value in D'quotes will be stored in a vector of bytes in Heap memory,
       and the first byte address will be stored in the String object created in stack.
       - 'String' implements 'Drop' trait which will be called just before string goes out of scope,
       1. clear off memory by deallocate the vector buffer on heap
       2. delete the string object on stack
*/

static _S_1: &str = "Hello"; // will only be deleted after the program ends

fn string_1() {
    let my_name = "Master"; // will be deleted when control goes out of function scope
    println!(
        "_S_1 from fn1\nValue: {} - Lenght: {} - FirstByte: {:p}",
        _S_1,
        _S_1.len(),
        _S_1.as_ptr()
    );
    println!(
        "Value: {} - Lenght: {} - FirstByte: {:p}",
        my_name,
        my_name.len(),
        my_name.as_ptr()
    );
}

fn string_2() {
    let mut _s_2: String = String::from(_S_1);
    println!(
        "Value: {} - Lenght: {} - FirstByte: {:p}",
        _s_2,
        _s_2.len(),
        _s_2.as_ptr()
    );
    _s_2.push_str(" World");
    println!(
        "Value: {} - Lenght: {} - FirstByte: {:p}",
        _s_2,
        _s_2.len(),
        _s_2.as_ptr()
    );
    let _s3: String = _s_2.trim().to_uppercase();
    println!(
        "Value: {} - Lenght: {} - FirstByte: {:p}",
        _s3,
        _s3.len(),
        _s3.as_ptr()
    );
} // all the 'String' types will be deleted by Drop() once the they are out of function scope

fn iterate_str_bytes(word: &str) {
    for letter in word.bytes() {
        println!("Decimal: {} - FirstByte: {:p}", letter, &letter);
    }
}

fn iterate_str_char(word: &str) {
    for letter in word.chars() {
        println!("Value: {} - FirstByte: {:p}", letter, &letter);
    }
}

pub fn do_it() {
    let name_1: &str = "Shiva";
    iterate_str_bytes(name_1);
    let mut name_2: String = String::from("Krishna");
    iterate_str_char(&name_2);
    let name_3: &mut str = &mut name_2;
    // ownership already borrowed to name_3. Hence it throws error.
    // Dosent work
    // iterate_str_char( name_2);
    // name_2.push('a');
    name_3.make_ascii_uppercase();
    println!("UpCase: {}", name_3);
    let some_word: &str = "abcdefghijk";
    println!(
        "Slicing some_word[..3] Value: {} - Lenght: {} - FirstByte: {:p}",
        &some_word[..3],
        &some_word[..3],
        &some_word[..3]
    );
    println!(
        "Slicing some_word[5..] Value: {} - Lenght: {} - FirstByte: {:p}",
        &some_word[5..],
        &some_word[5..],
        &some_word[5..]
    );
    println!(
        "Slicing some_word[4..7] Value: {} - Lenght: {} - FirstByte: {:p}",
        &some_word[4..7],
        &some_word[4..7],
        &some_word[4..7]
    );

    string_1();
    string_2();
    println!(
        "_S_1 from main\nValue: {} - Lenght: {} - FirstByte: {:p}",
        _S_1,
        _S_1.len(),
        _S_1.as_ptr()
    );
}
