use clap::ArgMatches;
use std::error::Error;
use crate::{
    Rask,
    net,
    app::{
        parser,
        errors::UncoveredError,
        status::{ Status, StatType }
    },
};

//don't need to worry about catching incorrect args because clap does it for us
pub fn dispatch_commands<'a>(
    matches: &'a ArgMatches, rask: &'a mut Rask
) -> Result<(), Box<dyn Error + 'a>> {
    if let Some(cmd) = matches.subcommand_name() {
        let subcmd_matches = matches.subcommand_matches(cmd).unwrap();
        if cmd == "group" || cmd == "namespace" || cmd == "note" || cmd == "tag" {
            let subcmd_name = subcmd_matches.subcommand_name().unwrap();
            let nest_subcmd_matches = subcmd_matches.subcommand_matches(subcmd_name).unwrap();
            match cmd {
                "group" => {
                    debug!("GROUP");
                    dispatch_group(nest_subcmd_matches, subcmd_name)?;
                } "namespace" => {
                    debug!("NAMESPACE");
                    dispatch_namespace(nest_subcmd_matches, subcmd_name)?;
                } "note" => {
                    debug!("NOTE");
                    dispatch_note(nest_subcmd_matches, subcmd_name)?;
                } "tag" => {
                    debug!("TAG");
                    dispatch_tag(nest_subcmd_matches, subcmd_name)?;
                } _ => {
                    return Err(Box::new(UncoveredError(cmd)));
                }
            };
        } else {
            match cmd {
                "add" => {
                    debug!("ADD");
                    rask.needs_saving = true;
                    let task_text = collect_arg_list(subcmd_matches, "task_text");
                    debug!("task_text: {:?}", task_text);
                    let task = parser::parse_task(task_text, rask.task_list.len() as u16 + 1);
                    match task {
                        Ok(t) => rask.task_list.push(t),
                        Err(e) => return Err(Box::new(e)),
                    }
                } "login" => {
                    debug!("LOGIN");
                    let login = subcmd_matches.get_one::<String>("login").unwrap();
                    let password = subcmd_matches.get_one::<String>("password").unwrap();
                    net::login(login.to_string(), password.to_string())?;
                    println!("Successful Login");
                    debug!("login: {:?}", login);
                    debug!("password: {:?}", password);
                } "register" => {
                    debug!("REGISTER");
                    let name = subcmd_matches.get_one::<String>("name").unwrap();
                    let login = subcmd_matches.get_one::<String>("login").unwrap();
                    let password = subcmd_matches.get_one::<String>("password").unwrap();
                    net::register(name.to_string(), login.to_string(), password.to_string())?;
                    println!("Successful Registration");
                    debug!("name: {:?}", name);
                    debug!("login: {:?}", login);
                    debug!("password: {:?}", password);
                } "done" => {
                    debug!("DONE");
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    debug!("task_index: {}", task_index);
                } "edit" => {
                    debug!("EDIT");
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    let override_text = collect_arg_list(subcmd_matches, "override_text");
                    debug!("task_index: {:?} | override_text: {:?}", task_index, override_text);
                } "list" => {
                    debug!("LIST");
                    let query_list = collect_arg_list(subcmd_matches, "query");
                    debug!("query_list: {:?}", query_list);
                    parser::parse_query(rask, query_list)?;
                    rask.disp();
                } "remove" => {
                    debug!("REMOVE");
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    debug!("task_index: {}", task_index);
                } "status" => {
                    debug!("STATUS");
                    rask.needs_saving = true;
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    let new_status_string = subcmd_matches.get_one::<String>("new_status").unwrap();
                    let new_status = Status::new(new_status_string.to_string(), StatType::Todo);
                    rask.update_status(*task_index, new_status)?;
                } "sync" => {
                    debug!("SYNC");
                } "push" => {
                    debug!("PUSH");
                    net::push(&rask.task_list)?;
                    println!("Successful Push");
                } "pull" => {
                    debug!("PULL");
                    rask.needs_saving = true;
                    let new_task_list = net::pull()?;
                    rask.task_list = new_task_list;
                    println!("Successful Pull");
                } "undo" => {
                    debug!("UNDO");
                } _ => {
                    return Err(Box::new(UncoveredError(cmd)));
                }
            };
        };
    }
    Ok(())
}

fn dispatch_note <'a>(matches: &ArgMatches, cmd: &'a str) -> Result<(), UncoveredError<'a>> {
    match cmd {
        "add" => {
            debug!("ADD");
            let task_index = matches.get_one::<u16>("task_index").unwrap();
            let note_text = matches.get_many::<String>("note_text")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>();
            debug!("task_index: {}", task_index);
            debug!("note_text: {:?}", note_text);
        } "list" => {
            debug!("LIST");
            let task_index = matches.get_one::<u16>("task_index").unwrap();
            debug!("task_index: {}", task_index);
        } "remove" => {
            debug!("REMOVE");
            let name = matches.get_one::<u16>("note_index").unwrap();
            debug!("name: {:?}", name);
        } _ => {
            debug!("NOT COVERED");
            return Err(UncoveredError(cmd));
        }
    }
    Ok(())
}

fn dispatch_group<'a>(matches: &ArgMatches, cmd: &'a str) -> Result<(), UncoveredError<'a>> {
    match cmd {
        "move" => {
            debug!("MOVE");
            let g_title = matches.get_one::<String>("group_title").unwrap();
            let ns_title = matches.get_one::<String>("namespace_title").unwrap();
            debug!("g title: {:?}", g_title);
            debug!("ns title: {:?}", ns_title);
        } "list" => {
            debug!("LIST");
        } "rename" => {
            debug!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            debug!("old: {:?}", old_name);
            debug!("new: {:?}", new_name);
        } _ => {
            return Err(UncoveredError(cmd));
        }
    }
    Ok(())
}

fn dispatch_namespace<'a>(matches: &ArgMatches, cmd: &'a str) -> Result<(), UncoveredError<'a>> {
    match cmd {
        "add" => {
            debug!("ADD");
            let name = matches.get_one::<String>("namespace_title").unwrap();
            debug!("name: {:?}", name);
        } "list" => {
            debug!("LIST");
        } "remove" => {
            debug!("REMOVE");
            let name = matches.get_one::<String>("namespace_title").unwrap();
            debug!("name: {:?}", name);
        } "rename" => {
            debug!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            debug!("old: {:?}", old_name);
            debug!("new: {:?}", new_name);
        } _ => {
            return Err(UncoveredError(cmd));
        }
    }
    Ok(())
}

fn dispatch_tag<'a>(matches: &ArgMatches, cmd: &'a str) -> Result<(), UncoveredError<'a>> {
    match cmd {
        "move" => {
            debug!("MOVE");
            let g_title = matches.get_one::<String>("group_title").unwrap();
            let ns_title = matches.get_one::<String>("namespace_title").unwrap();
            debug!("g title: {:?}", g_title);
            debug!("ns title: {:?}", ns_title);
        } "list" => {
            debug!("LIST");
        } "rename" => {
            debug!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            debug!("old: {:?}", old_name);
            debug!("new: {:?}", new_name);
        } _ => {
            return Err(UncoveredError(cmd));
        }
    }
    Ok(())
}

fn collect_arg_list<'a>(matches: &'a ArgMatches, id: &str) -> Vec<&'a str> {
    matches.get_many::<String>(id)
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
}
