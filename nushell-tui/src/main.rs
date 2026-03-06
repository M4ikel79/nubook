mod lesson;
mod ui;
mod executor;
mod progress;
mod hints;

use std::sync::Arc;
use anyhow::Result;
use tokio::sync::Mutex;

use lesson::LessonManager;
use progress::ProgressManager;
use ui::App;

fn main() -> Result<()> {
    let nubook_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../nubook".to_string());
    
    let lesson_manager = Arc::new(Mutex::new(LessonManager::new(&nubook_path)?));
    let progress_manager = Arc::new(Mutex::new(ProgressManager::load()?));
    
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut app = App::new(lesson_manager, progress_manager);
        app.run().await
    })
}
