use std::ffi::OsStr;

use yansi::Paint;

use crate::{Opts, OsDisplay};

pub trait Node {
    /// Return true if this node or any previous node contains fields.
    fn has_fields(&self) -> bool;

    /// Return true if this node or any previous node contains flags.
    fn has_flags(&self) -> bool;

    /// Return true if this node or any previous node contains parameters.
    fn has_params(&self) -> bool;

    /// Print help for commands on this node and all previous nodes.
    fn help_cmds(&self, has_fields: bool);

    /// Print help for fields on this node and all previous nodes.
    fn help_fields(&self, name: &OsStr);

    /// Print help for flags on this node and all previous nodes.
    fn help_flags(&self, has_fields: bool, name: &OsStr);

    /// Print help for parameters on this node and all previous nodes.
    fn help_params(&self, name: &OsStr);

    /// Print help text for this command
    fn help(&self);

    fn branch(&self, what: &OsStr, has_fields: bool, name: &OsStr) -> bool;
}

pub struct Help(pub(super) &'static str);

impl Help {
    pub(super) const fn new(help: &'static str) -> Self {
        Self(help)
    }
}

impl Node for Help {
    fn has_fields(&self) -> bool {
        false
    }

    fn has_flags(&self) -> bool {
        false
    }

    fn has_params(&self) -> bool {
        false
    }

    fn help_fields(&self, _name: &OsStr) {}

    fn help_cmds(&self, has_fields: bool) {
        if has_fields {
            println!(
                "   {}\n      Display this help message.",
                "--help".cyan().bright(),
            );
        } else {
            println!(
                "   {}, {}\n      Display this help message.",
                "help".cyan().bright(),
                "--help".cyan().bright(),
            );
        }
    }

    fn help_flags(&self, _has_fields: bool, _name: &OsStr) {}

    fn help_params(&self, _name: &OsStr) {}

    fn help(&self) {
        println!("{}\n", self.0);
    }

    fn branch(&self, _what: &OsStr, _has_fields: bool, _name: &OsStr) -> bool {
        false
    }
}

pub struct Cmd<T: Opts> {
    prev: T,
    name: &'static str,
}

impl<T: Opts> Cmd<T> {
    pub(super) const fn new(prev: T, name: &'static str) -> Self {
        Self { prev, name }
    }
}

impl<T: Opts> Node for Cmd<T> {
    fn has_fields(&self) -> bool {
        self.prev.has_fields()
    }

    fn has_flags(&self) -> bool {
        self.prev.has_flags()
    }

    fn has_params(&self) -> bool {
        self.prev.has_params()
    }

    fn help_fields(&self, name: &OsStr) {
        self.prev.help_fields(name)
    }

    fn help_cmds(&self, has_fields: bool) {
        self.prev.help_cmds(has_fields);

        if has_fields {
            println!(
                "   {}\n      FIXME.",
                format_args!("--{}", self.name).cyan().bright(),
            );
        } else {
            println!(
                "   {}, {}\n      FIXME.",
                self.name.cyan().bright(),
                format_args!("--{}", self.name).cyan().bright(),
            );
        }
    }

    fn help_flags(&self, has_fields: bool, name: &OsStr) {
        self.prev.help_flags(has_fields, name)
    }

    fn help_params(&self, name: &OsStr) {
        self.prev.help_params(name)
    }

    fn help(&self) {
        self.prev.help();
    }

    fn branch(&self, what: &OsStr, has_fields: bool, name: &OsStr) -> bool {
        if self.prev.branch(what, has_fields, name) {
            return true;
        }

        false
    }
}

pub(super) fn maybe_help(node: &impl Node, what: &OsStr, name: &OsStr) -> bool {
    let has_fields = node.has_fields();

    if !is_help(what, has_fields) {
        return false;
    }

    node.help();
    println!(
        "{}:\n   {} {}\n",
        "Usage".bold().bright().white(),
        format_args!("{}", OsDisplay(&name)).bright().blue(),
        "[OPTIONS] [COMMAND] [FIELDS]".bright().cyan(),
    );

    if has_fields {
        node.help_fields(name);
    }

    if node.has_flags() {
        node.help_flags(has_fields, name);
    }

    if node.has_params() {
        node.help_params(name);
    }

    println!("{}", "Commands:".bold().bright().white());
    node.help_cmds(has_fields);
    println!();

    true
}

fn is_help(what: &OsStr, has_fields: bool) -> bool {
    if has_fields {
        matches!(what.to_str(), Some("--help"))
    } else {
        matches!(what.to_str(), Some("help" | "--help"))
    }
}
