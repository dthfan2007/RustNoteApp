// @Author: Matteo Cipriani
// @Date:   04-06-2025 10:29:30
// @Last Modified by:   Matteo Cipriani
// @Last Modified time: 01-07-2025 09:04:39
//! # Note Module
//!
//! Defines the Note structure and related functionality for managing individual notes
//! including creation, modification tracking, and time formatting.

use chrono::{DateTime, Utc};
use chrono_tz::Europe::Zurich;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single note with metadata and content.
///
/// Each note has a unique ID, title, content, and timestamps for creation
/// and modification. All timestamps are stored in UTC and converted to
/// Swiss timezone for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Unique identifier for the note
    pub id: String,
    /// Display title of the note
    pub title: String,
    /// Main content/body of the note
    pub content: String,
    /// UTC timestamp when the note was created
    pub created_at: DateTime<Utc>,
    /// UTC timestamp when the note was last modified
    pub modified_at: DateTime<Utc>,
}

impl Note {
    /// Creates a new note with the given title.
    ///
    /// The note is initialized with an empty content, a unique UUID,
    /// and both created_at and modified_at set to the current time.
    ///
    /// # Arguments
    ///
    /// * `title` - The title for the new note
    ///
    /// # Returns
    ///
    /// * `Self` - A new Note instance
    ///
    /// # Examples
    ///
    /// ```
    /// let note = Note::new("My First Note".to_string());
    /// assert_eq!(note.title, "My First Note");
    /// assert!(note.content.is_empty());
    /// ```
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

    /// Updates the modification timestamp to the current time.
    ///
    /// This should be called whenever the note's content or title is changed.
    pub fn update_modified_time(&mut self) {
        self.modified_at = Utc::now();
    }

    /// Converts the creation timestamp to Swiss timezone.
    ///
    /// # Returns
    ///
    /// * `DateTime<chrono_tz::Tz>` - The creation time in Swiss timezone
    pub fn created_at_local(&self) -> DateTime<chrono_tz::Tz> {
        self.created_at.with_timezone(&Zurich)
    }

    /// Converts the modification timestamp to Swiss timezone.
    ///
    /// # Returns
    ///
    /// * `DateTime<chrono_tz::Tz>` - The modification time in Swiss timezone
    pub fn modified_at_local(&self) -> DateTime<chrono_tz::Tz> {
        self.modified_at.with_timezone(&Zurich)
    }

    /// Formats the modification time for display in Swiss timezone.
    ///
    /// Uses the format "DD.MM.YYYY HH:MM" which is common in Switzerland.
    ///
    /// # Returns
    ///
    /// * `String` - Formatted modification time string
    ///
    /// # Examples
    ///
    /// ```
    /// let note = Note::new("Test".to_string());
    /// let formatted = note.format_modified_time();
    /// // Returns something like "15.12.2024 14:30"
    /// ```
    pub fn format_modified_time(&self) -> String {
        self.modified_at_local()
            .format("%d.%m.%Y %H:%M")
            .to_string()
    }

    /// Formats the creation time for display in Swiss timezone.
    ///
    /// Uses the format "DD.MM.YYYY HH:MM" which is common in Switzerland.
    ///
    /// # Returns
    ///
    /// * `String` - Formatted creation time string
    pub fn format_created_time(&self) -> String {
        self.created_at_local().format("%d.%m.%Y %H:%M").to_string()
    }

    /// Generates a human-readable relative time description.
    ///
    /// Converts the time difference between now and the last modification
    /// into a user-friendly string like "2 hours ago" or "Yesterday".
    /// For very old notes, falls back to the absolute formatted time.
    ///
    /// # Returns
    ///
    /// * `String` - Relative time description
    ///
    /// # Examples
    ///
    /// ```
    /// let note = Note::new("Test".to_string());
    /// let relative = note.relative_time();
    /// // Returns "Just now" for a newly created note
    /// ```
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
