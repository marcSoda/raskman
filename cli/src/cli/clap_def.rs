use crate::BANNER;
use clap::{Command, Arg, ArgMatches};

// pub fn get_clap() -> Command<'static> {
pub fn get_clap() -> ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .before_help(BANNER)
    .after_help("https://github.com/marcSoda/raskman")
    .subcommand(namespace())
    .subcommand(group())
    .subcommand(note())
    .subcommand(status())
    .subcommand(add())
    .subcommand(done())
    .subcommand(remove())
    .subcommand(edit())
    .subcommand(list())
    .subcommand(login())
    .subcommand(register())
    .subcommand(sync())
    .subcommand(undo())
    .subcommand(tag())
    .get_matches()
}

fn list_template()  -> Command<'static> {
    Command::new("list")
        .visible_alias("l")
        .visible_alias("ls")
}

fn remove_template()  -> Command<'static> {
    Command::new("remove")
        .visible_alias("r")
        .visible_alias("rm")
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

fn add() -> Command<'static> {
    Command::new("add")
        .about("Add a new task")
        .visible_alias("a")
        .arg(Arg::new("task_text")
            .help("Task text")
            .required(true)
            .multiple_values(true)
            .takes_value(true))
}

fn done() -> Command<'static> {
    Command::new("done")
        .about("Mark a task as done")
        .visible_alias("d")
        .arg(Arg::new("task_index")
            .help("Index of task")
            .required(true)
            .value_parser(clap::value_parser!(u16))
            .takes_value(true))
}

fn remove() -> Command<'static> {
    remove_template()
        .about("Remove a task")
        .arg(Arg::new("task_index")
            .required(true)
            .takes_value(true)
            .value_parser(clap::value_parser!(u16))
            .help("Index of task to remove"))
}

fn edit() -> Command<'static> {
    Command::new("edit")
        .about("Edit a task")
        .visible_alias("e")
        .arg(Arg::new("task_index")
            .help("Index of task to override")
            .required(true)
            .value_parser(clap::value_parser!(u16))
            .takes_value(true))
        .arg(Arg::new("override_text")
            .help("Task override text")
            .required(true)
            .multiple_values(true)
            .takes_value(true))
}

fn list() -> Command<'static> {
    list_template()
        .about("List tasks")
        .arg(Arg::new("query")
            .required(false)
            .takes_value(true)
            .multiple_values(true)
            .help("Query text"))
}

fn login() -> Command<'static> {
    Command::new("login")
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

fn register() -> Command<'static> {
    Command::new("register")
        .about("Register with the server")
        .arg(Arg::new("name")
            .required(true)
            .takes_value(true)
            .help("name"))
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

fn undo() -> Command<'static> {
    Command::new("undo")
        .about("Undo last task action")
}

fn namespace() -> Command<'static> {
    Command::new("namespace")
        .about("Configure namespaces")
        .visible_alias("ns")
        .subcommand_required(true)
        .subcommand(rename_template()
            .about("Rename a namespace"))
        .subcommand(remove_template()
            .about("Remove a namespace")
            .arg(Arg::new("namespace_title")
                .required(true)
                .takes_value(true)
                .help("Title of namespace to remove")))
        .subcommand(list_template()
            .about("List all namespaces"))
        .subcommand(Command::new("add")
            .about("Add a new namespace")
            .visible_alias("a")
            .arg(Arg::new("namespace_title")
                .required(true)
                .help("Title of namespace to add")
                .takes_value(true)))
}

fn group() -> Command<'static> {
    Command::new("group")
        .about("Configure groups")
        .visible_alias("g")
        .subcommand_required(true)
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
        .subcommand_required(true)
        .subcommand(list_template()
            .arg(Arg::new("task_index")
                .help("index of task")
                .required(true)
                .value_parser(clap::value_parser!(u16))
                .takes_value(true)))
        .subcommand(Command::new("add")
            .about("Add a new note")
            .visible_alias("a")
            .arg(Arg::new("task_index")
                .help("index of task")
                .required(true)
                .value_parser(clap::value_parser!(u16))
                .takes_value(true))
            .arg(Arg::new("note_text")
                .help("text of the note")
                .required(true)
                .multiple_values(true)
                .takes_value(true)))
        .subcommand(remove_template()
            .about("Remove a note")
            .arg(Arg::new("note_index")
                .required(true)
                .takes_value(true)
                .value_parser(clap::value_parser!(u16))
                .help("Index of note to remove")))
}

fn status() -> Command<'static> {
    Command::new("status")
        .about("Change the status of a task")
        .visible_alias("s")
        .arg(Arg::new("task_index")
            .help("Index of task")
            .required(true)
            .value_parser(clap::value_parser!(u16))
            .takes_value(true))
        .arg(Arg::new("new_status")
            .help("Task's new status")
            .required(true)
            .takes_value(true))
}

fn tag() -> Command<'static> {
    Command::new("tag")
        .about("Configure tags")
        .visible_alias("t")
        .subcommand_required(true)
        .subcommand(list_template()
            .about("List all tags"))
        .subcommand(rename_template()
            .about("Rename tag"))
}
