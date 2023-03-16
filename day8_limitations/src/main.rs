/* The more I study, the more I see where Im falling short in understanding */


// - MultiAddr
// - SwarmEvent
// - Traits, Generics and Lifetime
// - Message passing between thread
// - Enum
// - Structs       ******
// - Select!
// - Defining new type using the "Type" Keyword
// - Patterns and matching

/* I will be learning Enum and Structs the rest of the day */

/* STRUCTS */

// Structs are like tuples, but they are different in a way we
// that we specify the name of each piece of data.

// We build the blueprint of a struct and instantiate it, by using 
// concrete data.


struct HumanBehaviour {
    brain: Vec<u32>,
}



/* ENUMS */

// This way we can specify types with different variants
// Struct provides us a way to group things, but an enum provide us a way
// to say this type can be a set of possible things.

// say we want to define an enumeration of activation functions

enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
}

// An interesting thing we can do, is put data into an enum.
// This data inside an enum and an enum variant can be seen as defining 
// a construction function.
// We can put anytype in our enum

/* I can say I have understand structs in a way */

fn main() {



    println!("Hello, world!");
}
