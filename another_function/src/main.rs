/*
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

fn main() {
    another_function(5 ,6);
}

fn another_function(x: i32, y: i8) {
    println!("The value of x is: {x} and {y}");
}


fn main() {
    // no let x = (let y = 6); error
    // esto si
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}


fn five() -> i32 {
   5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

*/
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}