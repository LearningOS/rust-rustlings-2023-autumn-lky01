// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
#[deny(clippy::almost_swapped)]
#[deny(clippy::panicking_unwrap)]
#[allow(unused_variables, unused_assignments)]
fn main() {
   

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

     

    let  value_a = 45;
    let  value_b = 66;
    // Let's swap these two!
    
    println!("value a: {}; value b: {}", value_a, value_b);
}
