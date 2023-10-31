// -----------------------------------------------------------------
// section 01. overview

/*
 */

// A `trait` is a collection of methods defined for an unknown type: `Self`. 
// They can access other methods declared in the same trait.
//
// Traits can be implemented for any data type. In the example below, we define `Animal`, a group of methods. 
// The `Animal trait` is then implemented for the `Sheep` data type, 
// allowing the use of methods from `Animal` with a `Sheep`.

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature: `Self` refers to the implementation type
    fn new( name: &'static str ) -> Self;

    // method signature: there will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }
    
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah!"
        } else {
            "haaaaaab!"
        }
    }

    // default trait mathods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());

    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // SAME ?????
    let mut molly = Sheep { name: "molly", naked: false };
    molly.talk();
    molly.shear();
    molly.talk();
}
