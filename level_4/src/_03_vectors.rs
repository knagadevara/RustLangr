/*
Vector is a generic, sequential, resizable collection of homogeneous elements.

Syntax:
let mut var_name : Vec<data-type> = Vec::new();
      or
let mut var_name = Vec::<data-type>::new();
ex: Creating a vector of type
let mut vector_1 = Vec::<u32>::new();

some of the functions which are available to use

push - adds element inside vector
pop - takes the last element out of array
insert - inserts an item in an index.
as the collection is sequential, length can be calculated.

While using 'for' to print elements in the vector it is important to use '&' operator to reference,
else the elements will be emptied out.
The complete contents of the vector can be printed using '{:?}' .

*/

pub fn demo_vectors() {
    // Creating a vector
    let mut vector_1 = Vec::<u32>::new();
    let mut vector_2: Vec<i32> = Vec::new();

    vector_1.push(10);
    vector_1.push(20);
    vector_1.push(30);
    vector_1.push(40);
    vector_1.push(50);

    vector_2.insert(0, 11);
    vector_2.insert(1, 21);
    vector_2.insert(2, 31);
    vector_2.insert(3, 41);
    vector_2.insert(4, 51);

    println!("Contents of Vector1: {:?}", vector_1);
    println!("Contents of Vector2: {:?}", vector_2);

    vector_1.pop();
    vector_2.pop();

    println!("\nAfter Popping!");
    println!("Vector1: {0:?}\nVector2: {1:?}", vector_1, vector_2);

    // for item in vector_1{ // without '&' the vector will delete all the values after printing.
    for item in &vector_1 {
        // '&' adds the functionality for borowing.
        println!("{:?}", item);
    }

    println!("Contents of Vector1: {:?}", vector_1); // Vector would be empty if '&' was not used.

    // safely get value from vector from any index
    let opt_var = vector_1.get(10);

    match opt_var {
        Some(vec_val) => print!("{:?}", vec_val),
        None => print!("0"),
    }
}
