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
    due_date: Option<String>,
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

    if let Some(date_str) = due_date {
        let parsed_due_date = Task::parse_due_date(&date_str)
            .ok_or_else(|| anyhow!("Invalid date format. Use YYYY-MM-DD"))?;
        task.update_due_date(Some(parsed_due_date));
    }

    storage.save_tasks(&tasks)?;
    println!("✓ Task {} updated", id);
    Ok(())
}

pub fn search_tasks(query: String) -> Result<()> {
    let storage = Storage::new()?;
    let tasks = storage.load_tasks()?;

    let query_lower = query.to_lowercase();
    let matching_tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|t| {
            t.title.to_lowercase().contains(&query_lower)
                || t.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
        })
        .collect();

    if matching_tasks.is_empty() {
        println!("No tasks found matching '{}'", query);
        return Ok(());
    }

    println!("\nFound {} task(s) matching '{}':", matching_tasks.len(), query);
    crate::display::print_tasks(&matching_tasks);
    Ok(())
}

pub fn show_stats() -> Result<()> {
    let storage = Storage::new()?;
    let tasks = storage.load_tasks()?;

    if tasks.is_empty() {
        println!("No tasks yet. Add one with 'task-cli add <title>'");
        return Ok(());
    }

    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let incomplete = total - completed;
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let high_priority = tasks.iter().filter(|t| t.priority == Priority::High).count();
    let medium_priority = tasks.iter().filter(|t| t.priority == Priority::Medium).count();
    let low_priority = tasks.iter().filter(|t| t.priority == Priority::Low).count();

    let overdue = tasks.iter().filter(|t| t.is_overdue()).count();

    use colored::Colorize;

    println!("\n{}", "═".repeat(50).bright_black());
    println!("{}", "Task Statistics".bold().cyan());
    println!("{}", "═".repeat(50).bright_black());
    println!("\n{}", "Overview:".bold());
    println!("  Total tasks:      {}", total.to_string().bold());
    println!("  Completed:        {} ({}%)", completed.to_string().green(), format!("{:.1}", completion_rate).green());
    println!("  Incomplete:       {}", incomplete.to_string().yellow());
    if overdue > 0 {
        println!("  Overdue:          {}", overdue.to_string().red().bold());
    }
    
    println!("\n{}", "By Priority:".bold());
    println!("  High:             {}", high_priority.to_string().red());
    println!("  Medium:           {}", medium_priority.to_string().yellow());
    println!("  Low:              {}", low_priority.to_string().cyan());
    println!("{}", "═".repeat(50).bright_black());
    println!();

    Ok(())
    println!("  High:             {}", high_priority.to_string().red());
    println!("  Medium:           {}", medium_priority.to_string().yellow());
    println!("  Low:              {}", low_priority.to_string().cyan());
    println!("{}", "═".repeat(50).bright_black());
    println!();

    Ok(())
}
