use crate::storage::Storage;
use crate::task::{Priority, Task};
use anyhow::{anyhow, Result};

pub fn add_task(
    title: String,
    description: Option<String>,
    priority: Option<String>,
    tags: Vec<String>,
    due_date: Option<String>,
) -> Result<()> {
    let storage = Storage::new()?;
    let mut tasks = storage.load_tasks()?;

    let priority = if let Some(p) = priority {
        Priority::from_str(&p)
            .ok_or_else(|| anyhow!("Invalid priority. Use: low, medium, or high"))?
    } else {
        Priority::Medium
    };

    let parsed_due_date = if let Some(date_str) = due_date {
        Some(Task::parse_due_date(&date_str)
            .ok_or_else(|| anyhow!("Invalid date format. Use YYYY-MM-DD"))?)
    } else {
        None
    };

    let id = storage.get_next_id()?;
    let task = Task::new(id, title, description, priority, tags, parsed_due_date);

    tasks.push(task.clone());
    storage.save_tasks(&tasks)?;

    println!("✓ Task added successfully (ID: {})", id);
    Ok(())
}

pub fn list_tasks(all: bool, filter: Option<String>, sort_by: Option<String>) -> Result<()> {
    let storage = Storage::new()?;
    let mut tasks = storage.load_tasks()?;

    // Apply filters
    if !all {
        tasks.retain(|t| !t.completed);
    }

    if let Some(filter_str) = filter {
        tasks.retain(|t| t.matches_filter(&filter_str));
    }

    // Sort tasks
    let sort_option = sort_by.as_deref().unwrap_or("priority");
    match sort_option {
        "date" => {
            tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }
        "priority" => {
            tasks.sort_by(|a, b| {
                b.priority.priority_value()
                    .cmp(&a.priority.priority_value())
                    .then(a.created_at.cmp(&b.created_at))
            });
        }
        "due-date" => {
            tasks.sort_by(|a, b| {
                match (a.due_date, b.due_date) {
                    (Some(d1), Some(d2)) => d1.cmp(&d2),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.created_at.cmp(&b.created_at),
                }
            });
        }
        _ => return Err(anyhow!("Invalid sort option. Use: date, priority, or due-date")),
    }

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    crate::display::print_tasks(&tasks);
    Ok(())
}

pub fn complete_task(id: u32) -> Result<()> {
    let storage = Storage::new()?;
    let mut tasks = storage.load_tasks()?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow!("Task with ID {} not found", id))?;

    if task.completed {
        println!("Task is already completed.");
        return Ok(());
    }

    task.complete();
    storage.save_tasks(&tasks)?;

    println!("✓ Task {} marked as complete", id);
    Ok(())
}

pub fn delete_task(id: u32) -> Result<()> {
    let storage = Storage::new()?;
    let mut tasks = storage.load_tasks()?;

    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);

    if tasks.len() == initial_len {
        return Err(anyhow!("Task with ID {} not found", id));
    }

    storage.save_tasks(&tasks)?;
    println!("✓ Task {} deleted", id);
    Ok(())
}

pub fn edit_task(
    id: u32,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<()> {
    let storage = Storage::new()?;
    let mut tasks = storage.load_tasks()?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow!("Task with ID {} not found", id))?;

    if let Some(new_title) = title {
        task.update_title(new_title);
    }

    if description.is_some() {
        task.update_description(description);
    }

    if let Some(p) = priority {
        let new_priority = Priority::from_str(&p)
            .ok_or_else(|| anyhow!("Invalid priority. Use: low, medium, or high"))?;
        task.update_priority(new_priority);
    }

    if let Some(new_tags) = tags {
        task.update_tags(new_tags);
    }

    storage.save_tasks(&tasks)?;
    println!("✓ Task {} updated", id);
    Ok(())
}
