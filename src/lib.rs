//! An opinonated argument parsing library that doesn't use macros.
//!
//! # Rules
//!
//! There are exactly four types of arguments:
//!  
//!  - [Flags](#flags)
//!  - [Parameters](#parameters)
//!  - [Commands](#commands)
//!  - [Fields](#fields-1)
//!
//! All command line programs must accept the command
//!
//!  - `--help` — Print help
//!
//! If there are no fields, they must also accept
//!
//!  - `help` — Alias to `--help`
//!
//! ## Flags
//!
//! Flags are single character lowercase ascii command line arguments that start
//! with `-`.  Multiple can be combined together.  Each flag may appear at most
//! once.
//!
//! ### Examples
//!
//! ```console
//! -v      Verbose
//! -f      Force
//! -vf     Verbose and force
//! ```
//!
//! ## Parameters
//!
//! Parameters are named arguments that can be passed in.  They must be a single
//! word, using only alphabetic ASCII without numbers.  List parameters are
//! defined by defining the parameter multiple times.
//!
//! ## Examples
//!
//! ```console
//! --verbosity {0…3}   Set verbosity level
//!     --verbosity 0
//!     --verbosity 1
//!     --verbosity 2
//!     --verbosity 3
//! --ratio <n>:<d>     Set ratio
//!     --ratio 1:2
//!     --ratio 5.3:7.1
//! [--append value]    Append a value
//!     --append 'book' --append 'car'
//! ```
//!
//! ## Commands
//!
//! Commands are a named subtree of CLI options.  Command names should be
//! lowercase alphabetic ASCII without numbers, using `-` for word separation.
//! No more than 3 words should be used, and 3 words should only if absolutely
//! necessary.
//!
//! Commands can start with `--` if there are possible positional arguments,
//! otherwise they should begin with an alphabetic character.  `--help` is
//! special in that it's required regardless of if there are positional
//! arguments.
//!
//! Lists of commands are also possible.
//!
//! ```console
//! --help
//! analyze
//! [exec <STMT>]
//!     exec 'a = 0' exec 'a += 1'
//! ```
//!
//! ## Fields
//!
//! Fields are positional arguments passed in to the program.
//!
//! ```console
//! <INT>   Integer
//!     42
//! <PATH>  Path - Only time when UTF-8 compliance is optional, depending on OS
//!     ~/my-files/something.text
//! ```

use std::{env, ffi::OsStr, fmt};

use yansi::Paint;

use self::node::{Help, Node};

mod node {
    use super::*;

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
                println!("{}", self.0);
                println!();
                println!("{}: {}", "Usage".bold(), OsDisplay(&name));
            }

            is_help
        }
    }
}

/// Command line option (sub)tree
pub struct Clot<T: Node = Help>(T);

impl Clot {
    /// Create a new command line argument option tree.
    ///
    ///  - `help` text describing what the command does
    pub fn new(help: &'static str) -> Self {
        Self(Help(help))
    }
}

impl<T: Node> Clot<T> {
    /// Create a new flag on the command.
    pub fn flag(self, flag: char) -> Self {
        if !flag.is_ascii_lowercase() {
            panic!("Flags must be ascii lowercase")
        }

        self
    }

    /// Create a new parameter on the command
    pub fn param(self) -> Self {
        self
    }

    /// Create a new subcommand.
    pub fn cmd<U: Node>(
        self,
        name: &'static str,
        help: &'static str,
        f: impl FnOnce() -> Clot<U>,
    ) -> Self {
        self
    }

    /// Create a new field on the subcommand
    pub fn field(self) -> Self {
        self
    }

    /// Validate the arguments and execute the selected subcommands.
    pub fn execute(self) {
        let mut iter = env::args_os();
        let name = iter.next().expect("Failed to get command name");
        let has_fields = false; // FIXME

        for arg in iter {
            if !self.0.branch(&arg, has_fields, &name) {
                println!(
                    "{}: Unexpected argument: `{}`",
                    "Error".red().bold(),
                    OsDisplay(&arg).bright().blue(),
                );
                println!();
                println!(
                    "       Try `{}` for more information.",
                    format_args!("{} --help", OsDisplay(&name)).bright().blue(),
                );
            }
        }
    }
}

struct OsDisplay<'a>(&'a OsStr);

impl fmt::Display for OsDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_string_lossy())
    }
}
