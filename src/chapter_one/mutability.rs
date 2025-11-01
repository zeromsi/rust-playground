
pub fn mutability(){
    let mut x = 5;
    println!("Value of X:{}", x);
    x = 10; // This line will give compilation error as x is by default immutable.
    println!("Value of X after change:{}", x);
}
