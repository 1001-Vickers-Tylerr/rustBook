fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing using let

    let y = 5;

    let y = y + 1;
    println!("The value of y is: {y}"); // outputs 6

    {
        let y = y * 2;
        println!("The value of y is: {y}"); // outputs 12
    }

    // Now, because of curly brackets (scope),
    // if we call println! again, it will output 6

    println!("The value of y is: {y}");
}


