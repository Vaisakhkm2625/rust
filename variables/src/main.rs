use std::{io, process};
use String;

fn main() {
    let a: u8 = 254;

    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    // let inp: u8 = inp.trim().parse().expect("input a number");
    let inp: u8 = match inp.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Enter a integer");
            process::exit(0x0100); 
            // ^this will exit program on wrong input than returning a int to inp
        }
    };


    let a: u8 = a + inp;

    println!("{}", a);
}

// let mut x = 5;
// println!("The value of x is: {x}");
// x = 6;
// println!("The value of x is: {x}");

// const MAX_POINTS: u32 = 100_000;
// println!("Max points is : {}",MAX_POINTS);

// // shtdowinng
// //
// let x = 5;
// let x = x+1;

// println!("The value of x is: {}",x);

// let spaces = "     ";
// let spaces = spaces.len();

// println!("length : {}",spaces);
