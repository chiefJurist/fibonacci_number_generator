use std::io;

//fibonacci function
fn fibonacci(n: u32) -> u64 {
    if n == 1 {
        return 0;
    }

    let mut previous: u64 = 0;
    let mut current: u64 = 1;

    for _ in 2..n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    current
}

//instruction function
fn instruction() {
    println!("Which Fibonacci Number Do You Want!");
    println!("Input 0  to exit 😉");
}

//ordinal number generator
fn ordinal(cardinal: u32) -> String {
    let suffix = match (cardinal % 100, cardinal % 10) {
        (11..=13, _) => "th",
        (_, 1) => "st",
        (_, 2) => "nd",
        (_, 3) => "rd",
        _ => "th",
    };

    format!("{cardinal}{suffix}")
}

fn main() {
    println!("Fibonacci Number Generator");

    loop {
        instruction();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let n: u32 = match input.trim().parse() {
            Ok(0) => break,
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a whole number!");
                continue;
            }
        };

        //94th Fibonacci Number is the largest number that fits in a u64
        if n > 94 {
            println!("That number is too big. Please choose 94 or less.");
            continue;
        }

        let result = fibonacci(n);
        let position = ordinal(n);
        println!("The {position} Fibonacci Number is {result}");
    }
}
