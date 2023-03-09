/* Smart pointer is just variable that holds address to memory location
 */

/* Smart pointers are structs that implement Deref and Drop trait */
/* The most strait forward smart pointer is Box<T>, this is like new 
keyword in c++ and malloc in c. It lets you store data in the heap */

/* My current understanding: Box<T> is currently the pointer to the type T
and Box::new(T) the address or the dereferencing of the type T  */

#[derive(Debug)]
struct Payment {
    name: String,
    amount: i32
}

#[derive(Debug)]
enum LinkedList {
    Node(Box<LinkedList>, Payment),
    Nil
}

#[derive(Debug)]
enum Graph {
    Node(Rc<Graph>, Payment)
    Nil
}

fn main() {
    let payment1 = Payment { name: String::from("John Achanya"), amount: 50000};
    let payment2 = Payment { name: String::from("Joe Achanya"), amount: 100000};
    let node = LinkedList::Node(Box::new(LinkedList::Node(Box::new(LinkedList::Nil), payment1)), payment2);


    let x = 5;
    let y: Box<i32> = Box::new(x);

    println!("y = {:#?}", y);
    println!("b = {:#?}", node);
}
