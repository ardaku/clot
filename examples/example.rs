use clot::{Clot, Opts};

fn main() {
    Clot::new("Example program")
        .cmd("hello", hello)
        .cmd("add", add)
        .execute()
}

fn add() -> Clot<impl Opts> {
    Clot::new("Add two numbers").run(run_add)
}

fn hello() -> Clot<impl Opts> {
    Clot::new("Print hello world").run(run_hello)
}

fn run_hello(_opts: &dyn Opts) {
    println!("Hello, world!");
}

fn run_add(_opts: &dyn Opts) {
    println!("Adding");
}
