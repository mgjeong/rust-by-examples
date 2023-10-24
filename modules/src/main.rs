// ---------------------------------------------------------------------------------------
// secton 01. visibility

/*
mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()` that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod`, that\n");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested`")
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_functions_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super)_mod`")
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_functions_in_super_mod();
    }

    // pub( create ) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified.
    //my_mod::nested::public_function_in_nested(); // ERROR...

    // private items of a module cannot be directly accessed, even if nested in a public method
    // my_mod::private_function(); // ERROR...
    // my_mod::nested::private_function(); // ERROR...
    // my_mod::private_nested::function(); // ERROR...
    // my_mod::private_nested::restricted_function(); // ERROR...
}
*/

// ---------------------------------------------------------------------------------------
// secton 02. struct visibility

/*
// Structs have an extra level of visibility with their fields. The visibility defaults to private,
// and can be overridden with the `pub`` modifier. This visibility only matters when a struct is accessed
// from outside the module where it is defined, and has the goal of hiding information (encapsulation).

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // a public struct with a private field of generic type `T`
    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        // a public constructor...
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents: contents }
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("the open box contains: {}", open_box.contents);

    // ERROR>..
    // let close_box = my::CloseBox {
    //     contents: "classified information",
    // };

    let _close_box = my::CloseBox::new("classified information");

    // println!("the closed box contains: {}", _close_box.contents); // ERROR...
}
*/

// ---------------------------------------------------------------------------------------
// secton 03. the use declaration
/*
// use crate::deeply::nested::{my_first_function, my_second_function, AndTraitType};

use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();

    println!("entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        function();

        println!("leaving block");
    }

    function();
}
*/

// ---------------------------------------------------------------------------------------
// secton 04. super and self
/*
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        println!("called `my::indirect_call()`, that\n> ");
        self::function();
        function();
        self::cool::function();
        cool::function();
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
*/

// ---------------------------------------------------------------------------------------
// secton 05. file hierarchy
//
// $ tree .
// +-- my --+-- inaccessible.rs
// |        +-- nested.rs
// +-- my.rs
// +-- split.rs
//
// $ rustc split.rs && ./split

fn main() {
    println!("this is to remove error.");
}
