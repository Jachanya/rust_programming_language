/* Project management is important and there is no doubt about that,
Almost every programming language implements a feature to package code
or organize large program. Packages makes it easier to arrange similar 
functionality*/

/* Rust projects are compiled as crates and there are two kinds of crates
which are binary crate and library crate. Binary crate must have a main file
and can be converted into an executable*/

/* Crate root is where the compiler starts from and a package contains one
or more crates */

/* Cargo follows a belief that the root crate for a binary project is
src/main.rs and the root crate for a library is src/lib.rs */

// ## Defining Modules to Control Scope and Privacy

/* How the compilers search for modules in human language
1. The compiler starts from the root crate
2. The compiler search for mod keyword.
    2.1. The compiler first check is the mod is inline
    2.2. Checks in the src/keyword.rs 
    2.3. check in the src/keyword/mod.rs
3. We can create sub module in any file other that the root crate
    3.1. The compiler will check if the submodule is inline.
    3.2. The compiler will check in src/parent_module/submodule.rs
    3.3. The compiler will check in src/parent_module/submodule/mod.rs
     */

pub mod payment {
    pub mod listener {
        pub fn add_to_event() { println!("It worked"); }
    }
}

/* Navigating through a module */
/* In other to navigate the module tree, we use a path like we
use in the file system. It can take absolute path (starts with "crate"
for path in current crate and for external crate it starts with crate_name)
and relative path. */

/* We can use the super keyword to access the parent module contents
like in file system with ../ */
fn main() {
    payment::listener::add_to_event();
    println!("Hello, world!");
}
