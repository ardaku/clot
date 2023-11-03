use clot::{Clot, Opts};

fn test() -> Clot<impl Opts> {
    Clot::new("Test command")
}

fn main() {
    Clot::new("Example program").cmd("test", test).execute()
}
