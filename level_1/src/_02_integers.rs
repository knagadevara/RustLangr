pub fn demo_integers(){
  let _a1: i32 = -1234; // unused variable should start with '_'
  let age: u8 = 25;
  let year: u16 = 2001;
  let hex_num: i32 = 0xFFFF;
  let oct_num: i32 = 0o177;
  let bin_num: i32 = 0b01101;
  
  // rust allows arch specific integers both signed and unsigned
  // isize and usize
  let unsigned_arch_def_isize: usize = 28;
  let signed_arch_def_usize: isize = 10001;

  // ways to display/print
  println!("\nAge: {}\nYear:{}", age, year);
  println!("\nHex: {0}\nOct: {1}\nBin: {2}\nUnsigned_Age:{3}", 
  hex_num, oct_num , bin_num , unsigned_arch_def_isize);
  println!("\nAge: {1}\nYear:{0}",year, signed_arch_def_usize);
  // Alllocated Size for Integers
  println!("\nSizeOf isize: {}", std::mem::size_of::<isize>());
  println!("\nSizeOf usize: {}", std::mem::size_of::<usize>());
  println!("\nSizeOf i32: {}", std::mem::size_of::<i32>());
  println!("\nSizeOf u32: {}", std::mem::size_of::<u32>());
}