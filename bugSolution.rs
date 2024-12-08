fn main() {
    let mut x = 5;
    { //This block limits the scope of the mutable borrow of y to prevent the error
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x;
    *z = 20; 
    println!("{}", x); // Output: 20
}