use day6_goods_tracking::consumers;

/// A system to build goods and food tracking
/// 
/// Important actors: 
///     - Consumers
///     - Merchants
///     - Sellers
/// 
/// The Merchants and the sellers commumicate using a P2P connections

fn main() {
    let consumer1 = consumers::Consumer::new(
        1234456,
        "John".to_owned(), 
        "Achanya".to_owned(),
        "Behind yoruba mosque".to_owned());

    let consumer1_name = consumer1.get_name();

    println!("Hello, world!: {}", consumer1_name);
}
