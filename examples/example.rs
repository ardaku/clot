use clot::{Clot, Opts};

fn test() -> Clot<impl Opts> {
    Clot::new("Test command", None)
}

fn main() {
    Clot::new("Example program", None)
        .cmd("test", test)
        .execute()
}
