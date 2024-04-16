fn main() {
    println!("Hello, world!");

    another_function();
    another_other_function(12);
    println!("{}", one_more_function(5.5));
    let x = ten();
    println!("{x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_other_function(x: i32) {
    println!("The value of x is: {x}");
}

fn one_more_function(x: f64) -> f64 {
    x
}

fn ten() -> i32 {
    10
}
