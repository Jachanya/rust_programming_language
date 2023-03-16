// Current Unsure information

// - MultiAddr
// - SwarmEvent
// - Traits, Generics and Lifetime
// - Message passing between thread
// - Select!                                      *****
// - Defining new type using the "Type" Keyword
// - Patterns and matching
// - Futures                                      *****


/* Future */
// A representation of a value that is not yet available, but would be in the future.
// Futures is simply a concept
// async fn download(url: Url) -> Result { .. }
// .await the execution is suspended


// Waiting on multiple features
// select! {
//     stream <- network.await => {

//     }

//     line <- terminal.await => {

//     }
// }
// select gives us like an octopus to branch different operations
// that can be awaited. Async await describe under what circumstances code 
// can make progress and under what circumstances code can yield.

fn main() {
    println!("Hello, world!");
}