fn main() {
    // floating points
    let x = 2.0; // f64

    let y: f32 = 3.0;

    let result = x / y;
    // Division: 1.7608695652173911
    println!("Division: {result}");

    let z = true;
    println!("True is {z}");

    let c = 'z';

    // compound types

    let tup = (500, 6.4, 1);
    // destructuring
    let (a, b, c) = tup;
    println!("{b}");
    // ... or access tuple's elements with '.'
    println!("{}", tup.0); // 500

    // initialiaze array by providing default value
    let arr = [3; 5];
    // special trait ':?' used to print the array (https://linux.how2shout.com/how-to-print-array-in-rust-lang/)
    println!("{:?}", arr);

}
