fn main() {
    println!("Hello, world!");

    another_function();
    another_function_again(5);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

// In function definitions, the type MUST be declared
fn another_function_again(x: i32) {
    println!("The value of x is: {x}.");
}

// Separate multiple parameters by comma
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}.");
}