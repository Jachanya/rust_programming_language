use std::cell::RefCell;
use std::ops::Mul;

//activity #7
enum Color {
    Red,
    Yellow,
    Green,
    Blue
}

//activity 8
#[derive(Debug)]
enum DrinkFlavor {
    Sugar,
    Sour,
}

#[derive(Debug)]
struct Drink {
    flavor: DrinkFlavor,
    fluid_volume: i32,
}

#[derive(Debug)]
struct ShippingBox {
    depth: i32,
    width: i32
}

fn main() {
    println!("Hello, world!");
    let two = RefCell::new(2);
    println!("The value two is {:#?}",   two);
    *two.borrow_mut() += 1;

    println!("The value two is {:#?}", two);
    let inp = vec![1,2,3,4];
    let output = matmul(&inp, &inp);
    println!("The output is {:#?}", output);

    display_name("John", "Achanya");
    display_sum(10, 16);
    display_message(true);
    display_match(false);
    for i in 1..10 {
        display_math_match(i);
    }

    let my_box = ShippingBox {
        depth: 50,
        width: 20
    };

    println!("{:#?}", my_box);

    let my_color = Color::Red;
    display_enum(my_color);

    let drink_1 = Drink{
        flavor: DrinkFlavor::Sugar,
        fluid_volume: 10
    };

    display_drink(drink_1);

    //activity 9
    let coord = cartesian(5, 2);
    match coord {
        (x, y) if y>5 => println!("Is greater than 5"),
        (x, y) if y<5 => println!("Is less than 5"),
        (x, y) if y==5 => println!("Is equal to 5"),
        (_, _) => println!("Unknown case"),
    }

    //activity 10
    display_expression(2000);
}

fn matmul<T: Mul + Mul<Output = T> + Copy>(a: & Vec<T>, b: & Vec<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for index in 0..a.len(){
        vec.push(a[index.clone()] * b[index.clone()]);
    }
    return vec;
}

//assignment #1
fn display_name(first_name: &str, last_name: &str){
    let name = format!("{first_name} {last_name}");
    println!("{}", name);
}

//assignment #2
fn display_sum(a: i32, b: i32) {
    println!("{a} + {b} = {}", a+b);
}

//assignment #3a
fn display_message(a: bool){
    if a {
        println!("hello");
    } else {
        println!("goodbye");
    }
}

//assignment #4a
fn display_match(value: bool){
    match value {
        true => println!("It's true"),
        false => println!("It's false"),
    }
}

//assignment #4b 
fn display_math_match(value: i32){
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other")
    }
}

//assignment #6 
fn display_countdown(value: &mut u32){
    while value > &mut 0 {
        println!("{value}");
        *value -= 1;
    }
    println!("done!");
}

//assignment #7
fn display_enum(value: Color){
    match value {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow")
    }
}

//assignment #8
fn display_drink(drink: Drink){
    println!("flavor {:#?}", drink.flavor);
    println!("ounces {}", drink.fluid_volume);
}

//assignment #9
fn cartesian(a: i32, b: i32) -> (i32, i32) {
    (a, b)
} 

//assignment #10
fn display_expression(a: i32) {

    let output_var = match a {
        a if a > 100 => "its big",
        a if a <= 100 => "its small",
        _ => "unknown",
    };
    println!("{output_var}");
}