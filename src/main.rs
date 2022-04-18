
fn main() {
    let mut x: i32;
    x = 10;
    x = 20;

    let x = 10;
    let (x, y) = (10, 20);

    const PI: f64 = 3.14;

    //Tuple
    let x = (1, 3.14, 1000);
    let x: (i32, f64, i32) = (1, 3.14, 1000);

    let a = x.0;

    //Array
    let x: [i32; 5];
    let x = [1, 2, 3, 4, 5];
    let x = [0; 5];

    let a = get_number();

    //if
    let score = 50;
    let mut grade = "";
    if score >= 80 {
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F"
    }

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    };

    // Condition || Ternary
    let result = if score >= 50 { "Pass" } else { "Fail" };

    // Loop
    while true {
        break;
    }

    // เหมือนกับ while true
    // ตั้งชื่อเป็น label1
    'label1: loop {
        'label2: loop {
            break 'label1;
            continue 'label2;
        }
    }

    // i = 0 i < 10 i++
    for i in 0..10 {
        println!("{}", i);
    }
}

// -> i32 ประกาศว่าจะ return i32
fn get_number() -> i32 {
    let a = 10;
    let b = 20;
    a + b // return แบบ tail expressions
}
