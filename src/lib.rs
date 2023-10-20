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

use std::env;

/// Command line argument option tree
pub struct Clot<T = &'static str>(T);

impl Clot {
    /// Create a new command line argument option tree.
    ///
    ///  - `help` text describing what the command does
    pub fn new(help: &'static str) -> Self {
        Self(help)
    }
}

impl<T> Clot<T> {
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
    pub fn cmd<U>(
        self,
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
        let iter = env::args_os();

        for arg in iter {
            println!("{arg:?}");
        }
    }
}
