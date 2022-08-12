use chrono::Utc;

use crate::app::{
    Rask,
    task::Task,
    group::Group,
    tag::Tag,
    status::{ Status, StatType },
    errors::SpecifierError,
};

use std::error::Error;

pub fn parse_task(task_list: Vec<&str>, num_tasks: u16) -> Result<Task, SpecifierError> {
    let mut task = Task {
        uuid: uuid::Uuid::new_v4().simple().to_string(),
        index: num_tasks,
        description: task_list.join(" "),
        groups: vec![],
        notes: vec![],
        tags: vec![],
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
            '+' => task.tags = parse_tag(&cmd[1..], task.tags)?,
            '%' => task.status = parse_status(&cmd[1..], task.clone())?,
            '@' => task.groups = parse_group(&cmd[1..], task.groups)?,
            _   => {
                let split: Vec<&str> = cmd.split(':').collect();
                if split.len() != 2 {
                    return Err(SpecifierError(cmd));
                }
                match split[0] {
                    "t" => task.tags = parse_tag(split[1], task.tags)?,
                    "s" => task.status = parse_status(split[1], task.clone())?,
                    "g" => task.groups = parse_group(split[1], task.groups)?,
                    // "time" => task.due = Some(parse_time(split[1], task.due)?),
                    // "date" => task.due = Some(parse_date(split[1], task.due)?),
                    _   => return Err(SpecifierError(cmd))
                };
            }
        };
    }

    Ok(task)
}

pub fn parse_query<'a>(rask: &mut Rask, query: Vec<&'a str>) -> Result<(), Box<dyn Error + 'a>> {
    let pq: Vec<&str> = query
        .into_iter()
        .filter(|w| w.contains(':')
                 || w.contains('+')
                 || w.contains('@')
                 || w.contains('%'))
        .collect();

    for spec in pq {
        //get first char to match on
        let first: char = match spec.chars().next() {
            Some(ch) => ch,
            None => return Err(Box::new(SpecifierError(spec))),
        };

        //match on the first char
        match first {
            '+' => filter_tags(rask, spec[1..].to_string()),
            '%' => filter_stat(rask, spec[1..].to_string()),
            '@' => filter_groups(rask, spec[1..].to_string()),
            _   => {
                let split: Vec<&str> = spec.split(':').collect();
                if split.len() != 2 {
                    return Err(Box::new(SpecifierError(spec)));
                }
                match split[0] {
                    "t" => filter_tags(rask, split[1].to_string()),
                    "g" => filter_groups(rask, split[1].to_string()),
                    "s" => filter_stat(rask, split[1].to_string()),
                    _   => return Err(Box::new(SpecifierError(spec)))
                };
            }
        };
    }
    Ok(())
}

fn filter_tags(rask: &mut Rask, tok: String) {
    rask.task_list = rask.task_list.clone()
        .into_iter()
        .filter(|t| t.tags_contains(tok.clone()))
        .collect::<Vec<Task>>();
}

fn filter_groups(rask: &mut Rask, tok: String) {
    rask.task_list = rask.task_list.clone()
        .into_iter()
        .filter(|t| t.groups_contains(tok.clone()))
        .collect::<Vec<Task>>();
}

fn filter_stat(rask: &mut Rask, tok: String) {
    rask.task_list = rask.task_list.clone()
        .into_iter()
        .filter(|t| t.stat_is(tok.clone()))
        .collect::<Vec<Task>>();
}

// fn parse_date(date_str: &str, mut due: Option<DateTime<Utc>>) -> Result<DateTime<Utc>, SpecifierError> {
//     Ok(Utc::now())
// }

// fn parse_time(time_str: &str, mut due: Option<DateTime<Utc>>) -> Result<DateTime<Utc>, SpecifierError> {
//     Ok(Utc::now())
// }

fn parse_tag(tag_name: &str, mut tags: Vec<Tag>) -> Result<Vec<Tag>, SpecifierError> {
    let tag = Tag::new(tag_name.to_string());
    tags.push(tag);
    Ok(tags)
}

fn parse_group(group_name: &str, mut groups: Vec<Group>) -> Result<Vec<Group>, SpecifierError> {
    let group = Group::new(group_name.to_string());
    groups.push(group);
    Ok(groups)
}

fn parse_status(status_name: &str, task: Task) -> Result<Status, SpecifierError> {
    if task.status != Status::default() {
        return Err(SpecifierError("Can't have multiple status specifiers."));
    }
    Ok(Status::new(status_name.to_string(), StatType::Todo))
    //TODO: THIS IS NOT RIGHT. should do more than just create a new status
    //should verify that given status exists then apply it
}
