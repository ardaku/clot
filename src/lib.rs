//! An opinionated mini argument parsing library that doesn't use macros.
//!
//! # Getting Started
//!
//! ```rust
#![doc = include_str!("../examples/example.rs")]
//! ```
//! 
//! # Rules
//!
//! Clot is opinionated on how you structure CLI arguments.  This is how they
//! work:
//!
//! There are exactly four types of arguments:
//!  
//!  - [Commands](#commands)
//!  - [Fields](#fields-1)
//!  - [Parameters](#parameters)
//!  - [Flags](#flags)
//!
//! All command line programs must accept the command
//!
//!  - `--help` — Print help
//!
//! If there are no fields, they must also accept
//!
//!  - `help` — Alias to `--help`
//!
//! ## Commands
//!
//! Commands are a named subtree of CLI options.  Command names should be
//! lowercase alphabetic ASCII without numbers, using `-` for word separation.
//! No more than 3 words should be used, and 3 words should only if absolutely
//! necessary.
//!
//! Commands must start with `--` if there are optional or required fields,
//! otherwise they must begin with an alphabetic character.  `--help` is special
//! in that it's required regardless of if there are fields.
//!
//! Lists of commands are also possible.
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
//! ```console
//! <INT>   Integer
//!     42
//! <PATH>  Path - Only time when UTF-8 compliance is optional, depending on OS
//!     ~/my-files/something.text
//! ```
//! 
//! ## Parameters
//!
//! Parameters are named arguments that can be passed in.  They must be a single
//! word, using only alphabetic ASCII without numbers.  List parameters are
//! defined by defining the parameter multiple times.
//!
//! ## Examples
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
//! ## Flags
//!
//! Flags are single character lowercase ascii command line arguments that start
//! with `-`.  Multiple can be combined together.  Each flag may appear at most
//! once.
//!
//! ### Examples
//! ```console
//! -v      Verbose
//! -f      Force
//! -vf     Verbose and force
//! ```

pub mod cmds;
pub mod flags;
mod node;
pub mod params;

use std::{
    env::{self, ArgsOs},
    ffi::{OsStr, OsString},
    fmt,
    iter::Peekable,
};

use yansi::Paint;

use self::node::{Cmd, Help, Node as Seal};

#[doc(hidden)]
pub enum Branch {
    Skip(Peekable<ArgsOs>),
    Help(Peekable<ArgsOs>),
    Done,
}

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
    pub fn new(help: &'static str) -> Self {
        Self {
            opts: Help::new(help),
            cmd_fn: None,
        }
    }
}

impl<T: Opts> Clot<T> {
    /// Add a callback to execute in place of help text when no subcomands are
    /// provided.
    pub fn run(mut self, f: CmdFn) -> Self {
        self.cmd_fn = Some(f);
        self
    }

    /// Create a new subcommand.
    ///
    /// # Panics
    ///
    ///  - If command `name` character is invalid (not lowercase ascii or `-`)
    ///  - If command `name` has more than two `-`
    ///  - If command `name` starts or ends with a `-`
    pub fn cmd<U: Opts, F: FnOnce() -> Clot<U>>(
        self,
        name: &'static str,
        f: F,
    ) -> Clot<Cmd<T, U, F>> {
        let invalid_char = |c: char| (!c.is_ascii_lowercase()) && c != '-';

        assert!(!name.contains(invalid_char));
        assert!(name.split_terminator('-').count() <= 3);
        assert!(!name.starts_with('-'));
        assert!(!name.ends_with('-'));

        Clot {
            opts: Cmd::new(self.opts, name, f),
            cmd_fn: self.cmd_fn,
        }
    }

    /// Create a new field on the subcommand
    pub const fn field(self) -> Self {
        self
    }

    /// Create a new parameter on the command
    pub const fn param(self, _name: &'static str) -> Self {
        self
    }

    /// Create a new flag on the command.
    pub const fn flag(self, flag: char) -> Self {
        if !flag.is_ascii_lowercase() {
            panic!("Flags must be ascii lowercase")
        }

        self
    }

    /// Validate the arguments and execute the selected subcommands.
    pub fn execute(self) {
        let mut iter = env::args_os().peekable();
        let name = iter.next().expect("Failed to get command name");

        self.execute_with(name, iter);
    }

    /// Execution of a specific subcommand
    fn execute_with(self, name: OsString, mut args: Peekable<ArgsOs>) {
        let has_fields = self.opts.has_fields();

        // If no arguments are provided to subcommand without command fn,
        // then display help
        if args.peek().is_none() && self.cmd_fn.is_none() {
            node::help(&self.opts, &name, has_fields);
        }

        while let Some(arg) = args.next() {
            // If passed `--help` or `help` when no fields, then display help.
            if node::maybe_help(&self.opts, &arg, &name, args.peek().is_some())
            {
                if let Some(arg) = args.next() {
                    unexpected(name, arg);
                }

                return;
            }

            args = match self.opts.branch(&arg, has_fields, &name, args) {
                Branch::Skip(args) => args,
                Branch::Help(_args) => {
                    unexpected(name, arg);
                    break;
                }
                Branch::Done => return,
            }
        }

        if let Some(cmd_fn) = self.cmd_fn {
            (cmd_fn)(&self.opts)
        }
    }
}

struct OsDisplay<'a>(&'a OsStr);

impl fmt::Display for OsDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_string_lossy())
    }
}

fn unexpected(name: OsString, arg: OsString) {
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
