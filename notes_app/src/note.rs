// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:30
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 04-06-2025 15:32:38
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Zurich;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            created_at: now,
            modified_at: now,
        }
    }

    pub fn update_modified_time(&mut self) {
        self.modified_at = Utc::now();
    }

    /// Get the created time in Swiss timezone
    pub fn created_at_local(&self) -> DateTime<chrono_tz::Tz> {
        self.created_at.with_timezone(&Zurich)
    }

    /// Get the modified time in Swiss timezone
    pub fn modified_at_local(&self) -> DateTime<chrono_tz::Tz> {
        self.modified_at.with_timezone(&Zurich)
    }

    /// Format the modified time for display in Swiss timezone
    pub fn format_modified_time(&self) -> String {
        self.modified_at_local()
            .format("%d.%m.%Y %H:%M")
            .to_string()
    }

    /// Format the created time for display in Swiss timezone
    pub fn format_created_time(&self) -> String {
        self.created_at_local().format("%d.%m.%Y %H:%M").to_string()
    }

    /// Get a relative time description (e.g., "2 hours ago")
    pub fn relative_time(&self) -> String {
        let now = Utc::now().with_timezone(&Zurich);
        let modified = self.modified_at_local();
        let duration = now.signed_duration_since(modified);

        if duration.num_seconds() < 60 {
            "Just now".to_string()
        } else if duration.num_minutes() < 60 {
            let minutes = duration.num_minutes();
            if minutes == 1 {
                "1 minute ago".to_string()
            } else {
                format!("{} minutes ago", minutes)
            }
        } else if duration.num_hours() < 24 {
            let hours = duration.num_hours();
            if hours == 1 {
                "1 hour ago".to_string()
            } else {
                format!("{} hours ago", hours)
            }
        } else if duration.num_days() < 7 {
            let days = duration.num_days();
            if days == 1 {
                "Yesterday".to_string()
            } else {
                format!("{} days ago", days)
            }
        } else if duration.num_weeks() < 4 {
            let weeks = duration.num_weeks();
            if weeks == 1 {
                "1 week ago".to_string()
            } else {
                format!("{} weeks ago", weeks)
            }
        } else {
            // For older notes, show the actual date
            self.format_modified_time()
        }
    }
}
