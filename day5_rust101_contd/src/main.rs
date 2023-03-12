// assignment 11
struct GroceryItem{
    quantity: i32,
    id: i32,
}

// assignment 12
#[derive(Debug)]
enum Color{
    Red,
    Yellow,
    Green,
    Blue,
    Orange,
}

#[derive(Debug)]
struct ShippingBox {
    dimension: i32,
    weight: i32,
    color: Color,
}

impl ShippingBox {
    fn new() -> Self {
        Self {
            dimension: 0,
            weight: 0,
            color: Color::Red,
        }
    }

    fn display(&self){
        println!("{:#?}", self);
    }
}

// assignment 14
struct Person{
    name: String,
    age: u32,
    color: String,
}

// assignment 15
enum Ticket {
    Backstage(i32),
    Vip(i32, String),
    Standard(i32, String),
}

// assignment 16
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {

    // assignment 11
    let item1 = GroceryItem {
        quantity: 50,
        id: 1
    };

    display_grocery(&item1);

    // assignment 12
    let shipping_box = ShippingBox::new();
    shipping_box.display();

    // assignment 13
    let vec_elem = vec![10,20,30,40];

    for elem in vec_elem{
        if elem == 30 {
            println!("thirty");
        }else{
            println!("{}", elem);
        }
    }

    // assignment 14
    let people = vec![
        Person {
            name: "John".to_owned(),
            age: 20,
            color: "Red".to_owned(),
        },
        Person {
            name: String::from("Joe"),
            age: 2,
            color: "Blue".to_owned(),
        },
        Person {
            name: String::from("Jerry"),
            age: 17,
            color: "Green".to_owned()
        },
        Person {
            name: String::from("Don"),
            age: 7,
            color: "Green".to_owned()
        }
    ];

    for person in people{
        if person.age <= 10 {
            println!("color: {:?}, name: {:?}", person.color, person.age);
        }
    }

    // assignment 15
    let ticket_holder = vec![
        Ticket::Backstage(1000),
        Ticket::Vip(2000, "Hose".to_owned()),
        Ticket::Standard(4000, "Eva".to_owned())];
    
    for ticket in ticket_holder{
        match ticket {
            Ticket::Backstage(price) => println!("Backstage ticket: #{price}"),
            Ticket::Vip(price, name) => println!("Vip Ticket: #{price}, name: {name}"),
            Ticket::Standard(price, name) => println!("Standard Ticket: #{price}, name: {name}"),
        }
    }

    // assignment 16
    let students = vec![
        Student{ name: String::from("John"), locker: Some(10)},
        Student{ name: String::from("Johnson"), locker: Some(110)},
        Student{ name: String::from("Joe"), locker: Some(20)},
        Student{ name: String::from("Jerry"), locker: Some(40)},
        Student{ name: String::from("Eva"), locker: None},
        ];
    
    for student in students {
        match student.locker {
            Some(value) => println!("The student {:#?} has a locker at {:#?}", student.name, student.locker),
            None => println!("The student {:#?} has no locker", student.name)
        }
    }
}

// assignment 11
fn display_grocery(item: &GroceryItem){
    println!("quantity: {}", item.quantity);
    println!("id: {}", item.id);
}