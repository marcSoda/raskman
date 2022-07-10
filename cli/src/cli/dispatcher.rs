use clap::ArgMatches;

//async?
//should return result eventually
//don't need to worry about catching incorrect args because clap does it for us
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
                let g_matches = matches.subcommand_matches(cmd).unwrap();
                let g_cmd = g_matches.subcommand_name().unwrap();
                let g_subcmd_matches = g_matches.subcommand_matches(g_cmd).unwrap();
                let g_subcmd_group_title = g_subcmd_matches.get_one::<String>("group_title").unwrap();
                let g_subcmd_ns_title = g_subcmd_matches.get_one::<String>("namespace_title").unwrap();
                println!("g title: {:?}", g_subcmd_group_title);
                println!("ns title: {:?}", g_subcmd_ns_title);
            } "list" => {
                println!("LIST");
            } "namespace" => {
                println!("NAMESPACE");
                let ns_matches = matches.subcommand_matches(cmd).unwrap();
                let ns_cmd = ns_matches.subcommand_name().unwrap();
                let ns_subcmd_matches = ns_matches.subcommand_matches(ns_cmd).unwrap();
                let ns_subcmd_title = ns_subcmd_matches.get_one::<String>("namespace_title").unwrap();
                println!("ns title: {:?}", ns_subcmd_title);
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

// fn handle_namespace(cmd: &str) {
//     match cmd {
//         "list" => {

//         }
//     }
// }
