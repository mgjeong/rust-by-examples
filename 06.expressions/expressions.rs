fn main() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x // expression.. assigned to the y
    };

    let z = {
        2 * x;
    }; // z is ()

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
