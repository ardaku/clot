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

pub mod cmds;
pub mod flags;
mod node;
mod os_str;
pub mod params;

use std::{
    env,
    ffi::{OsStr, OsString},
    fmt,
};

use yansi::Paint;

use self::node::{Cmd, Help, Node as Seal};
pub use self::os_str::FromOsStr;

type CmdFn = fn(&dyn Opts);

/// A sealed trait implemented on the generic of [`Clot`].
pub trait Opts: Seal {
    fn flag(&self, _c: char) -> bool {
        false
    }

    fn param(&self, _p: &str) -> Option<OsString> {
        None
    }

    fn field(&self, _f: usize) -> Option<OsString> {
        None
    }
}

impl<T: Seal> Opts for T {}

/// Command line option tree / subtree
pub struct Clot<T: Opts = Help> {
    opts: T,
    cmd_fn: Option<CmdFn>,
}

impl Clot {
    /// Create a new command line argument option tree.
    ///
    ///  - `help` text describing what the command does
    ///  - `f` function to execute when no subcommands are provided; use `None`
    ///    to show help text when no subcommands are provided
    pub fn new(help: &'static str, f: impl Into<Option<CmdFn>>) -> Self {
        Self {
            opts: Help::new(help),
            cmd_fn: f.into(),
        }
    }
}

impl<T: Opts> Clot<T> {
    /// Create a new flag on the command.
    pub const fn flag(self, flag: char) -> Self {
        if !flag.is_ascii_lowercase() {
            panic!("Flags must be ascii lowercase")
        }

        self
    }

    /// Create a new parameter on the command
    pub const fn param(self, _name: &'static str) -> Self {
        self
    }

    /// Create a new subcommand.
    pub fn cmd<U: Opts>(
        self,
        name: &'static str,
        _f: impl FnOnce() -> Clot<U>,
    ) -> Clot<Cmd<T>> {
        Clot {
            opts: Cmd::new(self.opts, name),
            cmd_fn: self.cmd_fn,
        }
    }

    /// Create a new field on the subcommand
    pub const fn field(self) -> Self {
        self
    }

    /// Validate the arguments and execute the selected subcommands.
    pub fn execute(self) {
        let mut iter = env::args_os();
        let name = iter.next().expect("Failed to get command name");
        let has_fields = false; // FIXME

        for arg in iter {
            if node::maybe_help(&self.opts, &arg, &name) {
                break;
            }

            if !self.opts.branch(&arg, has_fields, &name) {
                println!(
                    "{}: Unexpected argument `{}`\n",
                    "Error".red().bold(),
                    OsDisplay(&arg).bright().magenta(),
                );
                println!(
                    "       Try `{}` for more information.\n",
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
