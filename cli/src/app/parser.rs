use chrono::{
    DateTime,
    Utc,
};

use crate::app::{
    task::Task,
    group::Group,
    tag::Tag,
    status::{Status, StatType},
    errors::SpecifierError,
};

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
