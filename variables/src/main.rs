use std::io;

fn main() {
    //arrays
    //
    let months: [&str; 12] = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    // let tenstrings = ["geetha"; 10];

    // for i in months {
    //     println!("{}", i);
    // }

    // for i in tenstrings {
    //     println!("{}", i);
    // }

    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("unable to get input");

    let inp:usize = inp.trim().parse().expect("not a number");

    println!("{}", months[inp]);
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
//
//     let a: f32 = 0.3;
//     let b  = 0.2;

//    println!("0.3 + 0.2 = {}", a + b);
// // boolean
// let t: bool = true;
// let f: bool = false;
// println!("{} and {}",t,f);

// //tuple
//
// let tup = (10,10.3);
// // let tup: (i32,f64) = (10,10.3); // declaring types

// let (a,b) = tup;

// //println!("tuple {}", tup); // cannot print a tuple
// println!("a -\t{}\nb -\t{}", a,b);
// println!("tup.0 - \t{}\ntup.1 -\t{}", tup.0,tup.1);
