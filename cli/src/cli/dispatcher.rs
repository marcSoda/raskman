use clap::ArgMatches;

//async?
//should return result eventually?
//don't need to worry about catching incorrect args because clap does it for us
pub fn dispatch_commands(matches: &ArgMatches) {
    if let Some(cmd) = matches.subcommand_name() {
        let subcmd_matches = matches.subcommand_matches(cmd).unwrap();
        if cmd == "group" || cmd == "namespace" || cmd == "note" || cmd == "status" || cmd == "tag" {
            let subcmd_name = subcmd_matches.subcommand_name().unwrap();
            let nest_subcmd_matches = subcmd_matches.subcommand_matches(subcmd_name).unwrap();
            match cmd {
                "group" => {
                    println!("GROUP");
                    dispatch_group(nest_subcmd_matches, subcmd_name);
                } "namespace" => {
                    println!("NAMESPACE");
                    dispatch_namespace(nest_subcmd_matches, subcmd_name);
                } "note" => {
                    println!("NOTE");
                    dispatch_note(nest_subcmd_matches, subcmd_name);
                } "status" => {
                    println!("STATUS");
                    dispatch_status(nest_subcmd_matches, subcmd_name);
                } "tag" => {
                    println!("TAG");
                    dispatch_tag(nest_subcmd_matches, subcmd_name);
                } _ => {
                    println!("NOT COVERED");
                }
            };
        } else {
            match cmd {
                "add" => {
                    println!("ADD");
                    let task_text = collect_arg_list(subcmd_matches, "task_text");
                    println!("task_text: {:?}", task_text);
                } "auth" => {
                    println!("AUTH");
                    let login = subcmd_matches.get_one::<String>("login").unwrap();
                    let password = subcmd_matches.get_one::<String>("password").unwrap();
                    println!("login: {:?}", login);
                    println!("password: {:?}", password);
                } "done" => {
                    println!("DONE");
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    println!("task_index: {}", task_index);
                } "edit" => {
                    println!("EDIT");
                    let override_text = collect_arg_list(subcmd_matches, "override_text");
                    println!("override_text: {:?}", override_text);
                } "list" => {
                    println!("LIST");
                    let query = collect_arg_list(subcmd_matches, "query");
                    println!("note_text: {:?}", query);
                } "remove" => {
                    println!("REMOVE");
                    let task_index = subcmd_matches.get_one::<u16>("task_index").unwrap();
                    println!("task_index: {}", task_index);
                } "sync" => {
                    println!("SYNC");
                } _ => {
                    println!("NOT COVERED");
                }
            };

        };
    }
}

fn dispatch_note(matches: &ArgMatches, cmd: &str) {
    match cmd {
        "add" => {
            println!("ADD");
            let task_index = matches.get_one::<u16>("task_index").unwrap();
            let note_text = matches.get_many::<String>("note_text")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>();
            println!("task_index: {}", task_index);
            println!("note_text: {:?}", note_text);
        } "list" => {
            println!("LIST");
            let task_index = matches.get_one::<u16>("task_index").unwrap();
            println!("task_index: {}", task_index);
        } "remove" => {
            println!("REMOVE");
            let name = matches.get_one::<u16>("note_index").unwrap();
            println!("name: {:?}", name);
        } _ => {
            println!("NOT COVERED");
        }
    }
}

fn dispatch_status(matches: &ArgMatches, cmd: &str) {
    match cmd {
        "add" => {
            println!("add");
            let status_name = matches.get_one::<String>("status_name").unwrap();
            println!("status_name: {}", status_name);
        } "list" => {
            println!("LIST");
        } "remove" => {
            println!("RENAME");
            let status_name = matches.get_one::<String>("status_name").unwrap();
            println!("status_name: {}", status_name);
        } _ => {
            println!("NOT COVERED");
        }
    }
}

fn dispatch_group(matches: &ArgMatches, cmd: &str) {
    match cmd {
        "move" => {
            println!("MOVE");
            let g_title = matches.get_one::<String>("group_title").unwrap();
            let ns_title = matches.get_one::<String>("namespace_title").unwrap();
            println!("g title: {:?}", g_title);
            println!("ns title: {:?}", ns_title);
        } "list" => {
            println!("LIST");
        } "rename" => {
            println!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            println!("old: {:?}", old_name);
            println!("new: {:?}", new_name);
        } _ => {
            println!("NOT COVERED");
        }
    }
}

fn dispatch_namespace(matches: &ArgMatches, cmd: &str) {
    match cmd {
        "add" => {
            println!("ADD");
            let name = matches.get_one::<String>("namespace_title").unwrap();
            println!("name: {:?}", name);
        } "list" => {
            println!("LIST");
        } "remove" => {
            println!("REMOVE");
            let name = matches.get_one::<String>("namespace_title").unwrap();
            println!("name: {:?}", name);
        } "rename" => {
            println!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            println!("old: {:?}", old_name);
            println!("new: {:?}", new_name);
        } _ => {
            println!("NOT COVERED");
        }
    }
}

fn dispatch_tag(matches: &ArgMatches, cmd: &str) {
    match cmd {
        "move" => {
            println!("MOVE");
            let g_title = matches.get_one::<String>("group_title").unwrap();
            let ns_title = matches.get_one::<String>("namespace_title").unwrap();
            println!("g title: {:?}", g_title);
            println!("ns title: {:?}", ns_title);
        } "list" => {
            println!("LIST");
        } "rename" => {
            println!("RENAME");
            let old_name = matches.get_one::<String>("old_name").unwrap();
            let new_name = matches.get_one::<String>("new_name").unwrap();
            println!("old: {:?}", old_name);
            println!("new: {:?}", new_name);
        } _ => {
            println!("NOT COVERED");
        }
    }
}

fn collect_arg_list<'a>(matches: &'a ArgMatches, id: &str) -> Vec<&'a str> {
    matches.get_many::<String>(id)
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
}
