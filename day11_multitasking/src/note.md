### Multitasking

Only a single operation can run at a time on a single cpu core, In order to give the illusion that programs run at the same time the cpu switch between task and try to make progress on all of them.

## Async/Await in Rust

The rust programming language offers us with cooperative multitasking in the form of async/await

# Futures

Futures simply stated is a representation of a value that is not currently available.

The rust future trait has a method poll that returns an enum which verify if the future is ready or pending. The poll method takes to parameter cx and self. The cx is of type Context that pass a Waker type object into the poll function used to alert the future that the value being awaited for is ready, the self has a type Pin with a templete that is mutable reference to self.