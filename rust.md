#rust

let introduces a variable binding
```

let x;
x = 42;
```


#hello world

```

#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");

    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("Didn't recevice input");

    println!("Hello, {}! {}",name.trim_end(),greeting);
}
```