// Array is a fixed size of homogenious data-type collection
// Creating Arrays
// syntax for declarying
// let <array_name>: [ <data_type> ; <size> ] ; // add 'mut' for mutablity

pub fn demo_arrays(){
  // Creating an immutable array of type i32 with 5 elements
  // Size of the array cannot be changed(cannot add/delete).

  // Can be declared and initialized later.
  let ar1:[i32;5];
  ar1 = [0,1,2,3,4];
  // ar1[3] =9 ; // would throw error- not compile
  
  // declared and initialized with default values.
  let ar2 = [0 ; 5];

  // creaing a mutable array, only the elements will be changed but not the size.
  let mut ar3:[u8;3] = [0,0,0];

  // All the elements in array can be displayed using debug formating "{:?}"
  println!("\nAR1: {:?}",ar1);
  println!("\nAR2: {:?}",ar2);
  println!("\nAR3 Before: {:?}",ar3);
  ar3[0] = 1;
  ar3[1] = 2;
  ar3[2] = 0;
  println!("\nAR3 After: {:?}",ar3);

  // to print one element at a time.
  for elm in ar3{
    println!("Element {}", elm);
  }

}