use chrono::Utc;

use crate::app::{
    task::Task,
    group::Group,
    tag::Tag,
    status::{Status, StatType},
    errors::SpecifierError,
};

// pub fn get_task(task_list: Vec<&str>) -> Result<Task, SpecifierErr> {
pub fn parse_task(task_list: Vec<&str>, num_tasks: u16) -> Result<Task, SpecifierError> {
    let mut task = Task {
        uuid: uuid::Uuid::new_v4().simple().to_string(),
        index: num_tasks,
        description: task_list.join(" "),
        groups: None,
        notes: None,
        tags: None,
        status: Status::default(),
        due: None,
        created: Utc::now(),
        finished: None,
        recur: None,
        until: None,
    };

    let cmd_list: Vec<&str> = task_list
        .into_iter()
        .filter(|w| w.contains(':')
                 || w.contains('+')
                 || w.contains('@')
                 || w.contains('%'))
        .collect();

    for cmd in cmd_list {
        //get first char to match on
        let first: char = match cmd.chars().next() {
            Some(ch) => ch,
            None => return Err(SpecifierError(cmd)),
        };

        //match on the first char
        match first {
            '+' => { task.tags = Some(parse_tag(&cmd[1..], task.tags)?); },
            '%' => { task.status = parse_status(&cmd[1..])?; },
            '@' => { task.groups = Some(parse_group(&cmd[1..], task.groups)?); },
            _   => {
                let split: Vec<&str> = cmd.split(':').collect();
                if split.len() != 2 {
                    return Err(SpecifierError(cmd));
                }
                match split[0] {
                    "t" => { task.tags = Some(parse_tag(split[1], task.tags)?); },
                    "s" => { task.status = parse_status(split[1])?; },
                    "g" => { task.groups = Some(parse_group(split[1], task.groups)?); },
                    _   => return Err(SpecifierError(cmd))
                };
            }
        };
    }

    Ok(task)
}

//TODO: make sure you count things that should't happen twice like statues. or maybe just accept the last defined status?

fn parse_tag(tag_name: &str, tags: Option<Vec<Tag>>) -> Result<Vec<Tag>, SpecifierError> {
    let tag = Tag::new(tag_name.to_string());
    match tags {
        Some(mut t) => {
            t.push(tag);
            Ok(t)
        },
        None => Ok(vec![tag]),
    }
}

fn parse_group(group_name: &str, groups: Option<Vec<Group>>) -> Result<Vec<Group>, SpecifierError> {
    let group = Group::new(group_name.to_string());
    match groups {
        Some(mut g) => {
            g.push(group);
            Ok(g)
        },
        None => Ok(vec![group]),
    }
}

fn parse_status(status_name: &str) -> Result<Status, SpecifierError> {
    Ok(Status::new(status_name.to_string(), StatType::Todo))
    //THIS IS NOT RIGHT. should do more than just create a new status
    //should verify that given status exists then apply it
}
