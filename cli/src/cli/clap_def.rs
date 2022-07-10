use crate::BANNER;
use clap::{Command, Arg};

pub fn get_clap() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .before_help(BANNER)
    .after_help("https://github.com/marcSoda/raskman")
    .subcommand(namespace())
    .subcommand(group_subcommand())
    .subcommand(note())
    .subcommand(status_subcommand())
    .subcommand(add())
    .subcommand(done())
    .subcommand(remove())
    .subcommand(edit())
    .subcommand(list())
    .subcommand(auth())
    .subcommand(sync())
    .subcommand(tag_subcommand())
}

//note: there is no add_template because add is always different

fn list_template()  -> Command<'static> {
    Command::new("list")
        .visible_alias("l")
        .visible_alias("ls")
}

fn remove_template()  -> Command<'static> {
    Command::new("remove")
        .visible_alias("r")
        .visible_alias("rm")
        .arg(Arg::new("remove_index")
            .required(true)
            .takes_value(true))
}

fn rename_template()  -> Command<'static> {
    Command::new("rename")
        .visible_alias("r")
        .visible_alias("rn")
        .arg(Arg::new("old_name")
            .required(true)
            .takes_value(true)
            .help("Old Name"))
        .arg(Arg::new("new_name")
            .required(true)
            .takes_value(true)
            .help("New Name"))
}

//a limitation of clap is that we cannot accept multiple words as one arg
//instead of using clap, I will likely just accept the entire stdin and parse out the subcommands
fn add() -> Command<'static> {
    Command::new("add")
        .about("Add a new task")
        .visible_alias("a")
}

fn done() -> Command<'static> {
    Command::new("done")
        .about("Mark a task as done")
        .visible_alias("d")
        .arg(Arg::new("task_index")
            .help("index of task")
            .required(true)
            .takes_value(true))
}

fn remove() -> Command<'static> {
    remove_template()
        .about("Remove a task")
}

//same limitation as add()
//instead of using clap, I will likely just accept the entire stdin and parse out the subcommands
fn edit() -> Command<'static> {
    Command::new("edit")
        .about("Edit a task")
        .visible_alias("e")
}

//same limitation as add
//instead of using clap, I will likely just accept the entire stdin and parse out the subcommands
fn list() -> Command<'static> {
    list_template()
        .about("List tasks")
}

fn auth() -> Command<'static> {
    Command::new("auth")
        .about("Authenticate with the server")
        .arg(Arg::new("login")
            .required(true)
            .takes_value(true)
            .help("login"))
        .arg(Arg::new("password")
            .required(true)
            .takes_value(true)
            .help("password"))
}

fn sync() -> Command<'static> {
    Command::new("sync")
        .about("Synchronize with the server")
}

fn namespace() -> Command<'static> {
    Command::new("namespace")
        .about("Configure namespaces")
        .visible_alias("ns")
        .subcommand(Command::new("add")
            .about("Add a new namespace"))
            .visible_alias("a")
            .arg(Arg::new("ns_name")
                .required(true)
                .takes_value(true))
        .subcommand(remove_template()
            .about("Remove a namespace"))
        .subcommand(list_template()
            .about("List all namespaces"))
}

fn group_subcommand() -> Command<'static> {
    Command::new("group")
        .about("Configure groups")
        .visible_alias("g")
        .subcommand(list_template()
            .about("List all groups"))
        .subcommand(rename_template()
            .about("Rename group"))
        .subcommand(Command::new("move")
            .about("Move a group to a namespace")
            .visible_alias("m")
            .visible_alias("mv")
            .arg(Arg::new("group_title")
                .required(true)
                .takes_value(true)
                .help("title of group being moved"))
            .arg(Arg::new("namespace_title")
                .required(true)
                .takes_value(true)
                .help("title of namespace being mapped")))
}

fn note() -> Command<'static> {
    Command::new("note")
        .about("Configure notes")
        .visible_alias("n")
        .subcommand(list_template()
            .arg(Arg::new("task_index")
                .help("index of task")
                .required(true)
                .takes_value(true)))
        .subcommand(Command::new("add")
            .about("Add a new note"))
            .visible_alias("a")
            .arg(Arg::new("task_index")
                .help("index of task")
                .required(true)
                .takes_value(true))
        .subcommand(remove_template()
            .about("Remove a note"))
}

//TODO: make it so status can change task by task
fn status_subcommand() -> Command<'static> {
    Command::new("status")
        .about("Configure status list")
        .visible_alias("s")
        .subcommand(Command::new("add")
            .about("Add a new status"))
            .visible_alias("a")
            .arg(Arg::new("status_name")
                .required(true)
                .takes_value(true))
        .subcommand(remove_template()
            .about("Remove a status"))
        .subcommand(list_template()
            .about("List all statuses"))
}

fn tag_subcommand() -> Command<'static> {
    Command::new("tag")
        .about("Configure tags")
        .visible_alias("t")
        .subcommand(list_template()
            .about("List all tags"))
        .subcommand(rename_template()
            .about("Rename tag"))
}
