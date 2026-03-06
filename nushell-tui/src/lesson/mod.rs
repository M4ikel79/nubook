mod parser;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use parser::LessonParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub category: LessonCategory,
    pub content: Vec<ContentBlock>,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LessonCategory {
    GettingStarted,
    Fundamentals,
    Programming,
    AdvancedTopics,
    ShellFeatures,
    Reference,
}

impl LessonCategory {
    pub fn from_number(n: u32) -> Self {
        match n {
            1..=8 => LessonCategory::GettingStarted,
            9..=19 => LessonCategory::Fundamentals,
            20..=27 => LessonCategory::Programming,
            28..=36 => LessonCategory::AdvancedTopics,
            37..=47 => LessonCategory::ShellFeatures,
            48..=53 => LessonCategory::Reference,
            _ => LessonCategory::GettingStarted,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            LessonCategory::GettingStarted => "Getting Started",
            LessonCategory::Fundamentals => "Fundamentals",
            LessonCategory::Programming => "Programming",
            LessonCategory::AdvancedTopics => "Advanced Topics",
            LessonCategory::ShellFeatures => "Shell Features",
            LessonCategory::Reference => "Reference",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentBlock {
    Text(String),
    Code {
        language: String,
        code: String,
        expected_output: Option<String>,
    },
    Tip(String),
    Note(String),
    Header(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub description: String,
    pub initial_code: String,
    pub expected_output: Option<String>,
    pub hints: Vec<String>,
    pub validation_pattern: Option<String>,
}

pub struct LessonManager {
    lessons: HashMap<String, Lesson>,
    categories: HashMap<LessonCategory, Vec<String>>,
    nubook_path: String,
}

impl LessonManager {
    pub fn new(nubook_path: &str) -> Result<Self> {
        let parser = LessonParser::new(nubook_path);
        let (lessons, categories) = parser.parse_all()?;

        Ok(Self {
            lessons,
            categories,
            nubook_path: nubook_path.to_string(),
        })
    }

    pub fn get_lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    pub fn get_lessons_by_category(&self, category: &LessonCategory) -> Vec<&Lesson> {
        self.categories
            .get(category)
            .map(|ids| ids.iter().filter_map(|id| self.lessons.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_all_lessons(&self) -> Vec<&Lesson> {
        self.lessons.values().collect()
    }

    pub fn get_next_lesson(&self, current_id: &str) -> Option<Lesson> {
        let current = self.lessons.get(current_id)?;
        let category_lessons = self.categories.get(&current.category)?;

        if let Some(pos) = category_lessons.iter().position(|id| id == current_id) {
            if pos + 1 < category_lessons.len() {
                return self.lessons.get(category_lessons[pos + 1]).cloned();
            }
        }

        None
    }

    pub fn get_progress_path(&self) -> &str {
        &self.nubook_path
    }
}
