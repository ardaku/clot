use std::{cell::Cell, env::ArgsOs, ffi::OsStr, iter::Peekable};

use yansi::Paint;

use crate::{Branch, Clot, Opts, OsDisplay};

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

    /// Get help text for this command
    fn get_help_text(&self) -> &'static str;

    fn branch(
        &self,
        what: &OsStr,
        has_fields: bool,
        name: &OsStr,
        args: Peekable<ArgsOs>,
    ) -> Branch;
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
                "   {}\n      Display this help message",
                "--help".cyan().bright(),
            );
        } else {
            println!(
                "   {}, {}\n      Display this help message",
                "help".cyan().bright(),
                "--help".cyan().bright(),
            );
        }
    }

    fn help_flags(&self, _has_fields: bool, _name: &OsStr) {}

    fn help_params(&self, _name: &OsStr) {}

    fn get_help_text(&self) -> &'static str {
        self.0
    }

    fn branch(
        &self,
        _what: &OsStr,
        _has_fields: bool,
        _name: &OsStr,
        args: Peekable<ArgsOs>,
    ) -> Branch {
        Branch::Help(args)
    }
}

pub struct Cmd<T: Opts, U: Node, F: FnOnce() -> Clot<U>> {
    prev: T,
    name: &'static str,
    f: Cell<Option<F>>,
}

impl<T: Opts, U: Node, F: FnOnce() -> Clot<U>> Cmd<T, U, F> {
    pub(super) const fn new(prev: T, name: &'static str, f: F) -> Self {
        let f = Cell::new(Some(f));

        Self { prev, name, f }
    }
}

impl<T: Opts, U: Node, F: FnOnce() -> Clot<U>> Node for Cmd<T, U, F> {
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
        let help = (self.f.take().unwrap())().opts.get_help_text();

        self.prev.help_cmds(has_fields);

        if has_fields {
            println!(
                "   {}\n      {help}",
                format_args!("--{}", self.name).cyan().bright(),
            );
        } else {
            println!("   {}\n      {help}", self.name.cyan().bright());
        }
    }

    fn help_flags(&self, has_fields: bool, name: &OsStr) {
        self.prev.help_flags(has_fields, name)
    }

    fn help_params(&self, name: &OsStr) {
        self.prev.help_params(name)
    }

    fn get_help_text(&self) -> &'static str {
        self.prev.get_help_text()
    }

    fn branch(
        &self,
        what: &OsStr,
        has_fields: bool,
        name: &OsStr,
        args: Peekable<ArgsOs>,
    ) -> Branch {
        let args = match self.prev.branch(what, has_fields, name, args) {
            Branch::Skip(args) | Branch::Help(args) => args,
            Branch::Done => return Branch::Done,
        };

        let Some(what) = what.to_str() else {
            return Branch::Help(args);
        };

        let what = if has_fields {
            what.strip_prefix("--")
        } else {
            Some(what)
        };

        if what == Some(self.name) {
            (self.f.take().unwrap())()
                .execute_with(what.unwrap().to_string().into(), args);
            Branch::Done
        } else {
            Branch::Help(args)
        }
    }
}

pub(super) fn help(node: &impl Node, name: &OsStr, has_fields: bool) {
    let help_text = node.get_help_text();
    let options = if has_fields {
        format!(
            "{} {}\n",
            format_args!("{}", OsDisplay(&name)).bright().blue(),
            "[OPTIONS] [FIELDS] [OPTIONS]".bright().cyan(),
        )
    } else {
        String::new()
    };

    println!(
        "{help_text}\n\n{}:\n{}   {} {}\n",
        "Usage".bold().bright().white(),
        options,
        format_args!("{}", OsDisplay(&name)).bright().blue(),
        "[COMMAND] ...".bright().cyan(),
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
}

pub(super) fn maybe_help(
    node: &impl Node,
    what: &OsStr,
    name: &OsStr,
    dont_print: bool,
) -> bool {
    let has_fields = node.has_fields();

    if !is_help(what, has_fields) {
        return false;
    }

    if !dont_print {
        help(node, name, has_fields);
    }

    true
}

fn is_help(what: &OsStr, has_fields: bool) -> bool {
    if has_fields {
        matches!(what.to_str(), Some("--help"))
    } else {
        matches!(what.to_str(), Some("help" | "--help"))
    }
}
