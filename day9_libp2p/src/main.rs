/* Combining enum and struct  */

#[derive(Debug)]
enum Event {
    InBoundRequest {
        title: String,
        class: String,
    },
    OutBoundRequest {
        class: String,
    },
    StageTCP,
    Static(i32, i32, i32),
    Silencer,
}

fn main() {
    let variable = Event::InBoundRequest {title: String::from("Heaven"), class: String::from("ss2")};
    println!("Hello, world! {:#?}", variable);

    let var1 = Event::Static(12,22,33);

    if let Event::InBoundRequest {title, class} = variable {
        println!("{}", title);
    }

    match var1 {
        Event::InBoundRequest {title, class} => {
            println!("The InBoundRequestVariant title: {}, class: {}", title, class);
        },

        Event::StageTCP => {
            println!("This is stage tcp variant");
        },

        Event::Static(x,y,z) => {
            println!("Static({}, {}, {})", x,y,z)
        },
        _ => {}
    }
}
