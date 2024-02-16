/*
    'Copy' is a trait in Rust which would allow a bit-copy between two variables.
    Some of the Data-types implement 'Copy' but not all.
    ex: u8..u128 , i8..i128 , f32..f64 and &str .

    // would bit-copy a into b. Both the variables will exist in their scope.
    let a: u8 = 10;
    let b = a;

    // In case of 'Strings' structure,
    Fat-Pointer variable will be created on Stack,
    A muttable Byte-Vector will be created on Heap in which the literal will be stored.

    Fat-Pointer holds the address to the first byte of the byte-vector and size of literal.
    As String implements the Drop-trait the scope of the variable will end if it is assigned to another.
    ex: 'my_name' would lose its ownership of FatPointerOf-ByteVector to 'your_name'. Thus MOVED!
    let my_name: String = String::from("Master")
    let your_name = my_name;
*/

pub fn cp_mv_cln_demo() {
    let a = 10;
    println!("A Value: {:?} - Pointer: {:p}", a, &a);
    let b = a;
    println!("B Value: {:?} - Pointer: {:p}", b, &b);
    println!("A Value: {:?} - Pointer: {:p}", a, &a);

    // immutable static, function scope
    let s1 = "XYZ";
    println!(
        "Static String s1 Value: {:?} - Pointer: {:p}",
        s1,
        s1.as_ptr()
    );
    let s2 = s1;
    println!(
        "Static String s2 Value: {:?} - Pointer: {:p}",
        s2,
        s2.as_ptr()
    );
    println!(
        "Static String s1 Value: {:?} - Pointer: {:p}",
        s1,
        s1.as_ptr()
    );

    // String structure.
    let st1: String = String::from("FromString-1");
    println!(
        "String S1 Value: {} - Lenght: {} - FirstByte: {:p}",
        st1,
        st1.len(),
        st1.as_ptr()
    );
    let st2 = st1;
    println!(
        "let st2 = st1;
After the instruction St1 ownership on byte-vector will be moved to St2.
String S2 Value: {} - Lenght: {} - FirstByte: {:p}",
        st2,
        st2.len(),
        st2.as_ptr()
    );
    // println!("String S1 Value: {} - Lenght: {} - FirstByte: {:p}",st1, st1.len(), st1.as_ptr());
    // But clonning works
    let st3 = st2.clone();
    println!(
        "String S3 Value: {} - Lenght: {} - FirstByte: {:p}",
        st3,
        st3.len(),
        st3.as_ptr()
    );
    println!(
        "String S2 after cloning Value: {} - Lenght: {} - FirstByte: {:p}",
        st2,
        st2.len(),
        st2.as_ptr()
    );
}
