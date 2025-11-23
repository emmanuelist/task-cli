use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Priority::Low),
            "medium" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }

    pub fn priority_value(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        priority: Priority,
        tags: Vec<String>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            priority,
            tags,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    pub fn matches_filter(&self, filter: &str) -> bool {
        if let Some(priority_filter) = filter.strip_prefix("priority:") {
            return self.priority.as_str() == priority_filter.to_lowercase();
        }

        if let Some(tag_filter) = filter.strip_prefix("tag:") {
            return self.tags.iter().any(|t| t.to_lowercase() == tag_filter.to_lowercase());
        }

        if filter == "completed" {
            return self.completed;
        }

        if filter == "incomplete" {
            return !self.completed;
        }

        true
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn update_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }
}
