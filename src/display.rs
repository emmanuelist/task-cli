use crate::task::Task;
use colored::*;

pub fn print_tasks(tasks: &[Task]) {
    println!("\n{}", "═".repeat(100).bright_black());
    
    for task in tasks {
        print_task(task);
        println!("{}", "─".repeat(100).bright_black());
    }
}

pub fn print_task(task: &Task) {
    let status_symbol = if task.completed {
        "✓".green()
    } else {
        "○".yellow()
    };

    let priority_str = match task.priority {
        crate::task::Priority::High => "HIGH".red().bold(),
        crate::task::Priority::Medium => "MED".yellow(),
        crate::task::Priority::Low => "LOW".cyan(),
    };

    let id_str = format!("#{}", task.id).bright_black();
    
    println!("{} {} {} {}", 
        status_symbol, 
        id_str,
        priority_str,
        task.title.bold()
    );

    if let Some(desc) = &task.description {
        println!("   {}: {}", "Description".bright_black(), desc.white());
    }

    if !task.tags.is_empty() {
        let tags_str = task.tags
            .iter()
            .map(|t| format!("#{}", t))
            .collect::<Vec<_>>()
            .join(" ");
        println!("   {}: {}", "Tags".bright_black(), tags_str.blue());
    }

    let created = task.created_at.format("%Y-%m-%d %H:%M");
    println!("   {}: {}", "Created".bright_black(), created.to_string().bright_black());

    if let Some(due) = task.due_date {
        let due_str = due.format("%Y-%m-%d").to_string();
        if task.is_overdue() {
            println!("   {}: {} {}", "Due".bright_black(), due_str.red().bold(), "⚠ OVERDUE".red().bold());
        } else {
            println!("   {}: {}", "Due".bright_black(), due_str.cyan());
        }
    }

    if let Some(completed_at) = task.completed_at {
        let completed = completed_at.format("%Y-%m-%d %H:%M");
        println!("   {}: {}", "Completed".bright_black(), completed.to_string().green());
    }
}
