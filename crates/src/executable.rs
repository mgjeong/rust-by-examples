// $ rustc executable.rs --extern rary=library.rlib

fn main() {
    rary::public_function();

    // rary::private_function(); // ERROR...

    rary::indirect_access();
}