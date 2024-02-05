#[warn(dead_code)]
pub fn demo_if_else() {
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

    if your_grade >= A_GOLD_GRADE {
        println!("\n Congratulation!!!! Scored an A+, Gold!");
    } else if your_grade >= A_SILVER_GRADE && your_grade < A_GOLD_GRADE {
        println!("\n Congratulation!!!! Scored an A, Silver!");
    } else if your_grade >= A_BRONZE_GRADE && your_grade < A_SILVER_GRADE {
        println!("\n Congratulation!!!! Scored an A-, Bronze!");
    } else if your_grade >= B_GOLD_GRADE && your_grade < A_BRONZE_GRADE {
        println!("\n Congratulation!!!! Scored a B+!");
    } else if your_grade >= B_SILVER_GRADE && your_grade < B_GOLD_GRADE {
        println!("\n Can do Better!!!! Scored a B!");
    } else if your_grade >= B_BRONZE_GRADE && your_grade < B_SILVER_GRADE {
        println!("\n Need to Improve!!!! Scored a B-!");
    } else if your_grade >= C_GOLD_GRADE && your_grade < B_BRONZE_GRADE {
        println!("\n Focus on Studies!!!! Scored a C+!");
    } else if your_grade >= C_SILVER_GRADE && your_grade < C_GOLD_GRADE {
        println!("\n Focus and Improve!!!! Scored a C!");
    } else if your_grade >= D_GRADE && your_grade < C_SILVER_GRADE {
        println!("\n Improvement needed in all subjects!!!! D!");
    } else if your_grade >= E_GRADE && your_grade < D_GRADE {
        println!("\n Barely Passed, Improvement needed!!!! Promoted!");
    } else {
        println!("\nNot Qualified!");
    }
    println!("{0:#>5.2}", your_grade)
}
