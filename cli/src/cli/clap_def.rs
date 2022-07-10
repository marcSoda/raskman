use crate::BANNER;
use clap::{Command, Arg};

pub fn get_clap() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .before_help(BANNER)
    .after_help("github.com/marcSoda/taskman")
    .subcommand(namespace_subcommand())
    .subcommand(group_subcommand())
    .subcommand(note_subcommand())
    .subcommand(status_subcommand())
    .subcommand(add_subcommand())
    .subcommand(done_subcommand())
    .subcommand(remove_subcommand())
    .subcommand(edit_subcommand())
    .subcommand(list_subcommand())
    .subcommand(auth_subcommand())
    .subcommand(sync_subcommand())
    .subcommand(tag_subcommand())
}

pub fn add_subcommand() -> Command<'static> {
    Command::new("add")
        .visible_alias("a")
}

pub fn done_subcommand() -> Command<'static> {
    Command::new("done")
        .about("Mark a task as done")
        .visible_alias("d")
}

pub fn remove_subcommand() -> Command<'static> {
    Command::new("remove")
        .about("Remove a task")
        .visible_alias("rm")
        .visible_alias("r")
}

pub fn edit_subcommand() -> Command<'static> {
    Command::new("edit")
        .about("Edit a task")
        .visible_alias("e")
}

pub fn list_subcommand() -> Command<'static> {
    Command::new("list")
        .about("List tasks")
        .visible_alias("ls")
        .visible_alias("l")
}

pub fn auth_subcommand() -> Command<'static> {
    Command::new("auth")
        .about("Authenticate with the server")
}

pub fn sync_subcommand() -> Command<'static> {
    Command::new("sync")
        .about("Synchronize with the server")
}

pub fn namespace_subcommand() -> Command<'static> {
    Command::new("namespace")
        .about("Configure namespaces")
        .visible_alias("ns")
        .visible_alias("c")
        .visible_alias("context")
        .subcommand(add_subcommand()
            .about("Add a namespace")
            .arg(Arg::new("namespace title")
                .required(true)
                .takes_value(true)
                .value_name("NAMESPACE_TITLE")
                .help("title of namespace being created")
                .index(1))
        )
}

//only left here because I made every other subcommand nest it. delete later
fn namespace_add_subcommand() -> Command<'static> {
    Command::new("add")
        .about("Add a new namespace")
        .visible_alias("a")
        .arg(Arg::new("namespace title")
            .required(true)
            .takes_value(true)
            .value_name("NAMESPACE_TITLE")
            .help("title of namespace being created")
            .index(1))
}

pub fn group_subcommand() -> Command<'static> {
    Command::new("group")
        .about("Configure groups")
        .visible_alias("g")
        .subcommand(namespace_add_subcommand())
}

pub fn note_subcommand() -> Command<'static> {
    Command::new("note")
        .about("Configure notes")
        .visible_alias("n")
        .subcommand(namespace_add_subcommand())
}

pub fn status_subcommand() -> Command<'static> {
    Command::new("status")
        .about("Configure statuses")
        .visible_alias("s")
        .subcommand(namespace_add_subcommand())
}

pub fn tag_subcommand() -> Command<'static> {
    Command::new("tag")
        .about("Configure tags")
        .visible_alias("t")
        .subcommand(namespace_add_subcommand())
}
