fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; //Error: cannot borrow `x` as mutable more than once at a time
    *y = 10;
    *z = 10;
}