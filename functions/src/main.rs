fn main() {
    another_function();     
    another_function_1(23); 

    //expression
    let a = { 
       "geetha"
    };

    println!("{}",a);

    let add32 = add(3,2);

    println!(" adding 3 and 2 gives {}",add32);


    another_function_2(23, 43); // working
}


fn another_function (){
    println!("another function");

}


// no function overloading is possible

// fn another_function (x: i32 ){
//     println!("another function");
// }

fn another_function_1 (x: i32 ){
    println!("another function {}",x);

}

fn another_function_2(x: i32, y: i32) {
    println!("another function x {}", x);
    println!("another function y {}", y);
}

// no defult value is possible

// fn another_function (x: i32 = 23 ){
//     println!("another function");
// }


fn add(a: u32,b: u32) -> u32 {
    a+b
} 



fn generic_function <L,R>(x: L, y: R) {


}

