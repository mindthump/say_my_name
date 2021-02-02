fn main() {
    let name = "Mary";
    say_name(&name);
    say_name(&name);
}

fn say_name(name: &str) {
    println!("Hello, {}", name.to_string());
}
