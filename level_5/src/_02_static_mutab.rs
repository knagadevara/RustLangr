/*
  Rust allows Safe Multi threading possible with mutual exclution lock. 
  Resource would be available to only one thread and blocked for others until lock is released.
  As static-variable is available for multiple resources(statements/functions) in crate/module, 
  making it muttable with recommended way is by using 'unsafe' keyword (could cause chaos).
  
  //Declaring
  unsafe fn usf_exl(){
    static mut var_1: u32 = 0;
    var_1 += 1;
  }
  // calling using 'unsafe' wrapper.
  fn main(){
    unsafe {
      usf_exl();
    }
  }
 
  Highly Recommended way is to use Atomic types(defaultly mutable) to increment/decrement static variables.
 */
use std::sync::atomic::{AtomicU32, Ordering};

static mut USR_CNT: u32 = 0; 
static TTL_USR_CNT: AtomicU32 = AtomicU32::new(0); // note that there is no 'mut' keyword used

unsafe fn ulgin_add(){
  print!("Unsafe -> ");
  USR_CNT += 1;
  println!("User Logged In, GlobalUserCount: {:?}", USR_CNT);
}

unsafe fn ulgot_sub(){
  print!("Unsafe -> ");
  USR_CNT -=1;
  println!("User Logged Out, GlobalUserCount: {:?}", USR_CNT);
}

fn ulogin_add(){
  print!("Safe -> ");
  TTL_USR_CNT.fetch_add(1, Ordering::Relaxed );
  println!("User Logged In, GlobalUserCount: {:?}", TTL_USR_CNT);
}

fn ulogout_sub(){
  print!("Safe -> ");
  TTL_USR_CNT.fetch_sub(1, Ordering::Relaxed );
  println!("User Logged Out, GlobalUserCount: {:?}", TTL_USR_CNT);
}

pub fn do_it(){
  ulogin_add();
  ulogin_add();
  ulogin_add();
  ulogin_add();
  ulogout_sub();
  ulogin_add();
  ulogin_add();
  ulogin_add();
  ulogout_sub();
  ulogout_sub();
  ulogout_sub();
  
  // Unsafe Way of calling
  unsafe {
    ulgin_add();
    ulgin_add();
    ulgin_add();
    ulgin_add();
    ulgin_add();
    ulgot_sub();
    ulgin_add();
    ulgin_add();
    ulgot_sub();
    ulgin_add();
    ulgot_sub();
    ulgot_sub();
  }
}