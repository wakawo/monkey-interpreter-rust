use monkey_interpreter_rust::repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start().unwrap();
}
