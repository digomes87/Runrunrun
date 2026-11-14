// ==================== Variables ====================

pub fn variables() {
    let mut num_10: i32 = 10;
    let num_y = 9;
    let floating_point: f64 = 3.1415333;

    println!("Var: {}, var {}, and var {}", num_10, num_y, floating_point);

    // mutability: only num_10 can be reassigned
    num_10 = 2;

    // shadowing: num_y becomes a NEW variable of type &str
    let num_y = "ABC";

    let mut student_name: &str = "Antonia";
    println!("Student name: {student_name}");
    student_name = "Fagundes";
    println!("New student: {student_name}");

    println!("Now num 10 is: {} and num_y: {}", num_10, num_y);

    // shadowing inside a scope: only exists between the braces
    {
        let num_10 = "number 10";
        println!("inside a scope {num_10}");
    }
}
