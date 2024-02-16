// Tuples is a fixed-size hetrogeneous data-type collection.
// Only values can be changed but not the size and data-type

// Syntax
// let <tuple_name> = (v1 , "v2" , v3.0)
// mut can only change the value but not type.

pub fn demo_tuple() {
    let _tup0 = (); // empty typle
    let tup1: (u16, bool, f32); // declarying
    tup1 = (1, true, 1.5); // initializing
    let mut tup2 = tup1; // coping elements from other tuple.

    // assigning/updating values using index
    tup2.0 = 2;
    tup2.1 = false;
    tup2.2 = 2.5;

    // below would error out as tuples do not support iterations
    // for elem in tup2{
    //   println!("{}", elem) ;
    // }
    println!("{:?}", tup1);
    println!("{:?} - {:?} - {:?}", tup2.0, tup2.1, tup2.2);
}
