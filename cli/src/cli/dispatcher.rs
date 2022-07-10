use clap::ArgMatches;

//async?
//should return result eventually
pub fn dispatch_commands(matches: &ArgMatches) {
    if let Some(cmd) = matches.subcommand_name() {
        match cmd {
            "add" => {
                println!("ADD");
            } "auth" => {
                println!("AUTH");
            } "done" => {
                println!("DONE");
            } "edit" => {
                println!("EDIT");
            } "group" => {
                println!("GROUP");
            } "list" => {
                println!("LIST");
            } "namespace" => {
                println!("NAMESPACE");
                if let Some(ns_matches) = matches.subcommand_matches(cmd) {
                    if let Some(ns_cmd) = ns_matches.subcommand_name() {
                        println!("ns_cmd: {}", ns_cmd);
                    }
                }
            } "note" => {
                println!("NOTE");
            } "remove" => {
                println!("REMOVE");
            } "status" => {
                println!("STATUS");
            } "sync" => {
                println!("SYNC");
            } "tag" => {
                println!("TAG");
            } _ => {
                println!("NOT COVERED");
            }
        };
    }
}
