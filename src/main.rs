use ludndev_hello_world::greet;

fn main() {
    let name = "rustacean";
    let greeting = greet(name);
    println!("{}", greeting);
}
