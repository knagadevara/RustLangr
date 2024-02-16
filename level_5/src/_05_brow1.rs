/*
 Brorrowing ownership is leagal in Rust, achieved by using the '&'.
 Ownership is temporarily lended to the asking party, who has the authority
 to alter the original data.

 Rust strictly imposes the Lending and Borrowing of Ownership.
 Multiple Immutable Borrows(READ-ONLY).
 Single Muttable Borrow(WRITE).
*/

fn smp_borrow(br_var: &u8) {
    println!(
        "In function\nUnsigned Variable: {} - MemAdd: {:p}",
        br_var, br_var
    );
}

fn semi_mut_borrow(br_var: &String) {
    println!(
        "In function\nString Variable: {} - MemAdd: {:p}",
        br_var, br_var
    );
}

fn mut_explicit_borrow(br_var: &mut String) {
    br_var.push_str(" Bharath");
    println!(
        "In function\nMutable String Variable: {}  - MemAdd: {:p}",
        br_var, br_var
    );
}

pub fn simple_borrowing() {
    // Immutable referencing
    let tvar1: u8 = 10;
    println!(
        "1. Before Borrow\nUnsigned Variable: {} - MemAdd: {:p}",
        tvar1, &tvar1
    );

    // Rust allows many immutable references
    let b1var1: &u8 = &tvar1;
    println!(
        "2. First Borrow\nUnsignedVariable: {} - MemAdd: {:p}",
        b1var1, b1var1
    );

    let b2var1: &&u8 = &b1var1;
    println!(
        "3. Second Borrwo\nUnsigned Variable: {} - MemAdd: {:p}",
        b2var1, b2var1
    );

    smp_borrow(&b2var1);

    //Semi Mutable referencing is not allowed
    // Altrations are only possible before borrowing.
    let mut tvar3: String = String::from("Hello");
    println!(
        "Before Borrow\nUnaltered String: {} - MemAdd: {:p}",
        tvar3, &tvar3
    );
    tvar3.push_str(" World"); // Works before the borrow takes place
    println!(
        "After Borrow\nAltered String: {} - MemAdd: {:p}",
        tvar3, &tvar3
    );
    let c1var1 = &tvar3;
    println!(
        "After reborrow\nAltered String: {} - MemAdd: {:p}",
        c1var1, c1var1
    );
    // Now the write access will be lost for c1var1 and read-only ownership is with c1var1
    // hence it would not allow the original owner tvar3 to change anything.
    // tvar3.pop();// Compile Error.
    let c1var2 = &tvar3;
    let c1var3 = &c1var2; // double referencing allowed for immutable type only
    semi_mut_borrow(&c1var3);

    // Complete Mutable referencing
    let mut tvar4 = String::from("Namaste");
    mut_explicit_borrow(&mut tvar4); // Ownership comes back to tvar4 after function scope
    let mut tvar5 = &mut tvar4; // Muttable Ownership is given to tvars5.
                                // It will have the ablity to mutate original Value. At this point in time tvar4 looses complete ownership.
    tvar5.pop(); // As Ownership is with tvars6 this would be allowed.
    tvar5.clear(); // clears contents
    println!(
        "After Clear\naltered String: {} - MemAdd: {:p}",
        tvar5, tvar5
    );
    tvar5.push_str("Hello");
    mut_explicit_borrow(&mut tvar5); // Ownership comes back to tvar5 after function scope
    let tvar6 = &tvar5; // Immutable reference is given to tvar6 through tvar5
    let tvar7 = &tvar5; // Immutable reference is given to tvar7 through tvar5
                        // let tvar8 = &mut tvar5; // Would not work as immutable rfrence is already given to tvars6,7
                        // it cannot pass the mutablity to any other as it is a immutable variable.
                        // tvar7.pop(); // Dosent work
    semi_mut_borrow(tvar6); // only reading works.
                            // tvar4.push_str("Namaste");// Not allowed as Ownership is still with tvar5
    println!("Variation 3 String: {} - MemAdd: {:p}", tvar6, tvar6);
    println!("Variation 4 String: {} - MemAdd: {:p}", tvar7, tvar7);
    tvar5.push_str("G1"); // Works
    println!("Original Muttable String: {} - MemAdd: {:p}", tvar5, tvar5);
}
