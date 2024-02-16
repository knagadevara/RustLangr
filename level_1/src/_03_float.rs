// There are two types of floting point types f32 and f64
pub fn demo_float() {
    let electron_charge: f32 = -1.6021766e-16;
    let speed_of_light: f64 = 2.99792458e8;
    let my_salary: f32 = 2500.50;
    let total_revenue: f64 = 9999.999;

    // Printing regular floating number {} or {0}
    // Printing with Scientific notation {:e} or {1:e}
    // Printing float by setting allignment
    // {} Placeholder
    // :  Seperator
    // UTF-8 recognized symbol filler
    // <> Allignment Direction '<' is Left, '>' is Right
    // .N Presicion, numbers after decimal point
    // Example {2:->10.2} Right aligned
    // -----365.99
    // Example {3:#<10.2} Left aligned
    // 365.99#####

    println!(
        "\nTotal Revenue: {1:.4}\nMy Salary: {0:.2}",
        my_salary, total_revenue
    );
    println!("\nLeft Alligned\nTotal Revenue: {0:#<10.3}", total_revenue);
    println!("\nRight Alligned\nTotal Revenue: {0:#>10.3}", total_revenue);
    println!(
        "\nSpeed of Light\nNatural: {0}\nRestricted: {0:-<10.5e}",
        speed_of_light
    );
    println!(
        "\nElectron Charge\nNatural: {0}\nRestricted: {0:#>10.3e}",
        electron_charge
    );
}
