use std::io;

use rusty_frog::Frog;

fn main() {

    let some_frog: Frog = Frog::new(); // Frog::new() returns an instance of the Frog struct

    let mut input: String = String::new();
    let mut my_name: String = String::new();
    print!("Enter your frog's name: ");
    io::Write::flush(&mut io::stdout()).expect("Failed to flush buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");
    my_name = input.clone();
    input.clear();
    let my_weight: f64;
    print!("Enter your frog's weight: ");
    io::Write::flush(&mut io::stdout()).expect("Failed to flush buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");
    my_weight = input.trim().parse().unwrap();

    // You can define 'multiple constructors'! Kinda
    let mut my_frog: Frog = Frog::new_with_name_and_weight(my_name, my_weight); // Returns instance but is kind of like constructor overloading
    while my_frog.is_chilling() {
        my_frog.frog_life();
    }
}