use std::io;

fn main() {
    /*println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    another_function(10, 22);*/

    poop();
}

fn poop() {
    let x = get_x();

    let y = {
        let x = 3;
        x + 1
    };

    if x > 0 && y > 0 {
        println!("yes")
    } else {
        println!("no")
    }

    another_function(x, y);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn get_x() -> i32 {
    0
}