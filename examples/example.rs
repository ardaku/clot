use clot::{Clot, Opts};

fn run_hello(_opts: &dyn Opts) {
    println!("Hello, world!");
}

fn hello() -> Clot<impl Opts> {
    Clot::new("Print hello world").run(run_hello)
}

fn main() {
    Clot::new("Example program").cmd("hello", hello).execute()
}
