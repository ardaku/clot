use std::ffi::OsStr;

use yansi::Paint;

use crate::{Opts, OsDisplay};

pub trait Node {
    fn branch(&self, what: &OsStr, has_fields: bool, name: &OsStr) -> bool;
}

pub struct Help(pub(super) &'static str);

impl Node for Help {
    fn branch(&self, what: &OsStr, has_fields: bool, name: &OsStr) -> bool {
        let is_help = if has_fields {
            matches!(what.to_str(), Some("--help"))
        } else {
            matches!(what.to_str(), Some("help" | "--help"))
        };

        if is_help {
            println!("{}\n", self.0);
            println!("{}: {}\n", "Usage".bold(), OsDisplay(&name));
        }

        is_help
    }
}

impl<T: Node> Opts for T {}
