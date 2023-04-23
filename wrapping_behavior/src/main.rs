use std::{io, process};
use String;


// run this program with `cargo run --release` to see the wrapping behavior

fn main() {
    let a: u8 = 254;

    let mut inp = String::new();

    println!("Enter a number:");
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    // let inp: u8 = inp.trim().parse().expect("input a number");
    let inp: u8 = match inp.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("given input is not a integer");
            process::exit(0x0100);
            // ^this will exit program on wrong input than returning a int to inp
        }
    };

    println!("value of a is {}", a);
    println!("adding input to a {}", inp);
    let a: u8 = a + inp;

    println!("value of a is {}", a);

}
