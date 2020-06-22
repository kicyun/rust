
fn main() {
    let x = 5;

    let x = x + 1;
    let x = x + 2;
    
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    //spaces = spaces.len();

    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("heart_eyed_cat {}", heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let siz_point_four = x.1;

    let one = x.2;

    println!("x is ({}, {}, {})", five_hundred, siz_point_four, one);

    // The Array Type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let index = 1;

    let element = a[index];

    println!("The value of element is : {}", element);

    let index = 10;

    // let element = a[index];
    //  println!("The value of element is : {}", element);

    another_function(5);

    another_function2(5, 6);

    bind_test();

    let x = five();

    println!("The value of x is : {}", x);

    let x = plus_one(5);
    println!("The value of x is : {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is : {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn bind_test() {
    let x = 5;
    let y = {
        let x = 3;
        x+1
    };
    println!("the value of y is : {}", y);
}