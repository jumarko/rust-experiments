fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // If you try this, it will print error "cannot assign twice to immutable variable `x`"
    // x = 6;
    // println!("The value of x is {x}");

    // But we can "shadow variable by using `let` again - even with a diferent type"
    let spaces = "   ";
    let spaces= spaces.len();
    println!("spaces: {spaces}")
}

