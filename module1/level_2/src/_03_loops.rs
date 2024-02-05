// Type of loops and their keywords

pub fn rust_loops() {
    // 	while -> loop with a condition to come out off.
    // requires a mutable variable
    let mut idx: usize = 0;
    while idx < 10 {
        println!("{}", idx);
        idx += 1; // remember ++ and -- do not exist in rust
    }

    //	for-in -> runs over an arra of elements
    let my_marks: [u8; 7] = [90, 75, 89, 42, 100, 95, 97];
    for element in my_marks {
        if element < 45 {
            println!("\nDo not bother Printing!");
            continue;
        }
        if element == 100 {
            println!("\nImma Champp!!");
            break;
        }
        println!("\n{}", element);
    }

    //	for -> general for with a range (inclusive, excelusive)
    // for idx in 1..10{ //exclusive the outer limit
    for idx in 1..=10 {
        // inclusive outer limit
        println!("{}", idx)
    }

    // Nestead Loop, loops can be named anything in this case named it as 'outer_loop'
    'outer_loop: loop {
        println!("Outer Loop");

        'inner_loop: for i in 1..=3 {
            println!("Inner Loop {}", i);
            if i == 3 {
                break 'inner_loop;
            }
        }
        break 'outer_loop;
    }

    // 	loop -> Infinite loop. Code in the braces would { run forever } as ther is no condition to break it.
    // loop{
    // 	println!("\nRunning Service!");
    // 	std::thread::sleep(std::time::Duration::from_secs(1));
    // }
}
