#[path="./hello.rs"] mod hello;

fn soma(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let name: &str = "World";
    let _idade: i64 = 18i64;
    let _cores: Vec<&str> = vec!["red", "green"];
    println!("{}", soma(10, 10));
    hello::print_hello(name)
}