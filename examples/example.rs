use clot::{Clot, Opts};

fn run_test(_opts: &dyn Opts) {
    println!("Running the test");
}

fn test() -> Clot<impl Opts> {
    Clot::new("Test command").run(run_test)
}

fn main() {
    Clot::new("Example program").cmd("test", test).execute()
}
