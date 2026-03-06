use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub completed_lessons: HashMap<String, LessonProgress>,
    pub total_xp: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub last_session: Option<DateTime<Utc>>,
    pub total_sessions: u32,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonProgress {
    pub lesson_id: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub attempts: u32,
    pub hints_used: u32,
    pub xp_earned: u32,
    pub first_try_success: bool,
}

impl Default for UserProgress {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            completed_lessons: HashMap::new(),
            total_xp: 0,
            current_streak: 0,
            longest_streak: 0,
            last_session: None,
            total_sessions: 0,
            started_at: Utc::now(),
        }
    }
}

pub struct ProgressManager {
    progress: UserProgress,
    save_path: PathBuf,
}

impl ProgressManager {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nushell-tui");

        fs::create_dir_all(&config_dir)?;

        let save_path = config_dir.join("progress.json");
        let progress = if save_path.exists() {
            let content = fs::read_to_string(&save_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            UserProgress::default()
        };

        Ok(Self {
            progress,
            save_path,
        })
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.progress)?;
        fs::write(&self.save_path, content)?;
        Ok(())
    }

    pub fn get_progress(&self) -> &UserProgress {
        &self.progress
    }

    pub fn complete_lesson(
        &mut self,
        lesson_id: &str,
        hints_used: u32,
        first_try: bool,
    ) -> Result<u32> {
        let xp_base: u32 = 100;
        let xp_hint_penalty: u32 = hints_used * 10;
        let xp_first_try_bonus: u32 = if first_try { 50 } else { 0 };
        let xp_earned: u32 = xp_base.saturating_sub(xp_hint_penalty) + xp_first_try_bonus;

        let lesson_progress = self
            .progress
            .completed_lessons
            .entry(lesson_id.to_string())
            .or_insert_with(|| LessonProgress {
                lesson_id: lesson_id.to_string(),
                completed: false,
                completed_at: None,
                attempts: 0,
                hints_used: 0,
                xp_earned: 0,
                first_try_success: false,
            });

        lesson_progress.attempts += 1;
        lesson_progress.hints_used += hints_used;

        if !lesson_progress.completed {
            lesson_progress.completed = true;
            lesson_progress.completed_at = Some(Utc::now());
            lesson_progress.xp_earned = xp_earned;
            lesson_progress.first_try_success = first_try;
            self.progress.total_xp += xp_earned;
        }

        self.update_streak()?;
        self.save()?;

        Ok(xp_earned)
    }

    pub fn is_lesson_completed(&self, lesson_id: &str) -> bool {
        self.progress
            .completed_lessons
            .get(lesson_id)
            .map(|p| p.completed)
            .unwrap_or(false)
    }

    pub fn get_completed_count(&self) -> usize {
        self.progress
            .completed_lessons
            .values()
            .filter(|p| p.completed)
            .count()
    }

    fn update_streak(&mut self) -> Result<()> {
        let today = Utc::now().date_naive();

        if let Some(last) = self.progress.last_session {
            let last_date = last.date_naive();
            let days_diff = (today - last_date).num_days();

            match days_diff {
                0 => {}
                1 => {
                    self.progress.current_streak += 1;
                }
                _ => {
                    self.progress.current_streak = 1;
                }
            }
        } else {
            self.progress.current_streak = 1;
        }

        self.progress.longest_streak = self
            .progress
            .longest_streak
            .max(self.progress.current_streak);
        self.progress.last_session = Some(Utc::now());
        self.progress.total_sessions += 1;

        Ok(())
    }
}
