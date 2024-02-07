#[warn(dead_code)]

pub fn demo_match() {
    const A_GOLD_GRADE: u8 = 95;
    const A_SILVER_GRADE: u8 = 90;
    const A_BRONZE_GRADE: u8 = 85;

    const B_GOLD_GRADE: u8 = 80;
    const B_SILVER_GRADE: u8 = 75;
    const B_BRONZE_GRADE: u8 = 70;

    const C_GOLD_GRADE: u8 = 65;
    const C_SILVER_GRADE: u8 = 60;

    const D_GRADE: u8 = 50;
    const E_GRADE: u8 = 40;

    let your_grade: u8 = 71;

    match your_grade {
        A_GOLD_GRADE | A_SILVER_GRADE | A_BRONZE_GRADE => println!("\nA Grade"),
        B_GOLD_GRADE | B_SILVER_GRADE | B_BRONZE_GRADE => println!("\nB Grade"),
        C_GOLD_GRADE | C_SILVER_GRADE => println!("\nC Grade"),
        D_GRADE => println!("\nAverage Grade"),
        E_GRADE => println!("\nPassed."),
        _ => println!("\nNot Qualified."),
    }

    match your_grade {
        A_GOLD_GRADE..=100 => println!("\nA Grade\nGOLD!"),
        A_SILVER_GRADE..=94 => println!("\nA Grade\nSILVER!"),
        A_BRONZE_GRADE..=89 => println!("\nA Grade\nBRONZ!"),
        B_GOLD_GRADE..=84 => println!("\nB Grade\nGOLD!"),
        B_SILVER_GRADE..=79 => println!("\nB Grade\nSILVER!"),
        B_BRONZE_GRADE..=74 => println!("\nB Grade\nBRONZ!"),
        C_GOLD_GRADE..=69 => println!("\nC Grade\nGOLD!"),
        C_SILVER_GRADE..=64 => println!("\nC Grade\nSILVER!"),
        D_GRADE..=59 => println!("\nAverage Grade"),
        E_GRADE..=49 => println!("\nPassed."),
        _ => println!("\nNot Qualified."),
    }

    match your_grade {
        // compiler will create a temporary variable with the contents match-argument
        // in this case it would be "let grd: u8 = your_grade;"
        grd if grd >= A_GOLD_GRADE => {
            println!("\n Congratulation!!!! Scored an A+, Gold!");
        }
        grd if grd >= A_SILVER_GRADE && grd < A_GOLD_GRADE => {
            println!("\n Congratulation!!!! Scored an A, Silver!");
        }
        grd if grd >= A_BRONZE_GRADE && grd < A_SILVER_GRADE => {
            println!("\n Congratulation!!!! Scored an A-, Bronze!");
        }
        grd if grd >= B_GOLD_GRADE && grd < A_BRONZE_GRADE => {
            println!("\n Congratulation!!!! Scored a B+!");
        }
        grd if grd >= B_SILVER_GRADE && grd < B_GOLD_GRADE => {
            println!("\n Can do Better!!!! Scored a B!");
        }
        grd if grd >= B_BRONZE_GRADE && grd < B_SILVER_GRADE => {
            println!("\n Need to Improve!!!! Scored a B-!");
        }
        grd if grd >= C_GOLD_GRADE && grd < B_BRONZE_GRADE => {
            println!("\n Focus on Studies!!!! Scored a C+!");
        }
        grd if grd >= C_SILVER_GRADE && grd < C_GOLD_GRADE => {
            println!("\n Focus and Improve!!!! Scored a C!");
        }
        grd if grd >= D_GRADE && grd < C_SILVER_GRADE => {
            println!("\n Improvement needed in all subjects!!!! D!");
        }
        grd if grd >= E_GRADE && grd < D_GRADE => {
            println!("\n Barely Passed, Improvement needed!!!! Promoted!");
        }
        _ => {
            println!("\nNot Qualified!");
        }
    }

    println!("{0:#>5.2}", your_grade)
}
