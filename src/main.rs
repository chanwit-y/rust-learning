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
    let x = [0;5];
}
