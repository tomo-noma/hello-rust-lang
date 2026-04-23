fn main() {
    let greeting = greet();
    println!("{}!!", greeting);
}

fn greet() -> String {
    let greeting = format!("Hello, {}!", "Alice");
    println!("{}", greeting);
    greeting
}
