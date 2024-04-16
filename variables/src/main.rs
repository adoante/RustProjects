fn main() {
    let x = 5;

    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{spaces}");

    let x = 2.0;
    let y: f32 = 3.0;

    println!("f64 (double): {x}");
    println!("f32 (float): {y}");

    let c = 'z';
    let z: char = 'Z';
    let emoji = '😴';

    println!("{c}{z}{emoji}");

    let tup: (i32, f64, u8) = (400, 4.6, 1);
    let (x, y, z) = tup;
    println!("({x},{y},{z})");

    let a = [3; 5];
    println!("{}",a[1]);
}
