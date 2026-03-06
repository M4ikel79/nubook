use crate::executor::{NuExecutor, ValidationResult};
use crate::lesson::{ContentBlock, LessonCategory};
use crate::progress::ProgressManager;
use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarState},
    Frame, Terminal,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::lesson::LessonManager;

pub struct App {
    lesson_manager: Arc<Mutex<LessonManager>>,
    progress_manager: Arc<Mutex<ProgressManager>>,
    current_screen: Screen,
    current_category_index: usize,
    current_lesson_index: usize,
    categories: Vec<LessonCategory>,
    lesson_scroll: u16,
    output_scroll: u16,
    input_buffer: String,
    hints_revealed: u32,
    current_exercise_index: usize,
    last_output: String,
    last_error: String,
    show_hints: bool,
    completed_exercise: bool,
    xp_earned: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
enum Screen {
    Menu,
    Lesson,
    Exercise,
    Completed,
}

impl App {
    pub fn new(
        lesson_manager: Arc<Mutex<LessonManager>>,
        progress_manager: Arc<Mutex<ProgressManager>>,
    ) -> Self {
        let categories = vec![
            LessonCategory::GettingStarted,
            LessonCategory::Fundamentals,
            LessonCategory::Programming,
            LessonCategory::AdvancedTopics,
            LessonCategory::ShellFeatures,
            LessonCategory::Reference,
        ];
        
        Self {
            lesson_manager,
            progress_manager,
            current_screen: Screen::Menu,
            current_category_index: 0,
            current_lesson_index: 0,
            categories,
            lesson_scroll: 0,
            output_scroll: 0,
            input_buffer: String::new(),
            hints_revealed: 0,
            current_exercise_index: 0,
            last_output: String::new(),
            last_error: String::new(),
            show_hints: false,
            completed_exercise: false,
            xp_earned: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        terminal.clear()?;
        
        loop {
            terminal.draw(|f| self.render(f))?;
            
            if let Some(key) = self.handle_input().await {
                match key {
                    "q" => break,
                    "esc" => {
                        self.current_screen = Screen::Menu;
                        self.reset_state();
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    async fn handle_input(&mut self) -> Option<String> {
        use std::io::Read;
        
        let mut stdin = std::io::stdin();
        let mut buf = [0u8; 1];
        
        if stdin.read(&mut buf).ok()? == 0 {
            return None;
        }
        
        match self.current_screen {
            Screen::Menu => self.handle_menu_input(buf[0]),
            Screen::Lesson => self.handle_lesson_input(buf[0]).await,
            Screen::Exercise => self.handle_exercise_input(buf[0]).await,
            Screen::Completed => self.handle_completed_input(buf[0]).await,
        }
    }

    fn handle_menu_input(&mut self, key: u8) -> Option<String> {
        match key {
            b'j' => {
                self.current_category_index = (self.current_category_index + 1) % self.categories.len();
                None
            }
            b'k' => {
                self.current_category_index = if self.current_category_index == 0 {
                    self.categories.len() - 1
                } else {
                    self.current_category_index - 1
                };
                None
            }
            b'\n' => {
                self.current_lesson_index = 0;
                self.current_screen = Screen::Lesson;
                None
            }
            b'q' => Some("q".to_string()),
            _ => None,
        }
    }

    async fn handle_lesson_input(&mut self, key: u8) -> Option<String> {
        match key {
            b'j' => {
                self.lesson_scroll = (self.lesson_scroll + 1).min(100);
                None
            }
            b'k' => {
                self.lesson_scroll = self.lesson_scroll.saturating_sub(1);
                None
            }
            b'\n' => {
                self.current_screen = Screen::Exercise;
                self.current_exercise_index = 0;
                None
            }
            b'q' | 27 => Some("q".to_string()),
            _ => None,
        }
    }

    async fn handle_exercise_input(&mut self, key: u8) -> Option<String> {
        if self.completed_exercise {
            return self.handle_completed_input(key).await;
        }
        
        match key {
            b'r' => {
                self.reveal_hint();
                None
            }
            b':' => {
                self.current_screen = Screen::Lesson;
                None
            }
            127 => {
                self.input_buffer.pop();
                None
            }
            b'\n' => {
                self.run_exercise().await;
                None
            }
            27 => Some("esc".to_string()),
            _ => {
                if key >= 32 && key < 127 {
                    self.input_buffer.push(key as char);
                }
                None
            }
        }
    }

    async fn handle_completed_input(&mut self, key: u8) -> Option<String> {
        match key {
            b'\n' => {
                self.go_to_next_lesson();
                None
            }
            27 | b'q' => Some("q".to_string()),
            _ => None,
        }
    }

    fn render(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>) {
        let size = f.area();
        
        match &self.current_screen {
            Screen::Menu => self.render_menu(f, size),
            Screen::Lesson => self.render_lesson(f, size),
            Screen::Exercise => self.render_exercise(f, size),
            Screen::Completed => self.render_completed(f, size),
        }
        
        self.render_status_bar(f, size);
    }

    fn render_menu(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        let block = Block::default()
            .title(" Nushell TUI - Learn Nushell Interactively ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        
        f.render_widget(block, size);
        
        let inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2));
        
        let title = Paragraph::new("Welcome to Nushell TUI!")
            .style(Style::default().fg(Color::Yellow).bold());
        f.render_widget(title, inner[0]);
        
        let mut list_items = Vec::new();
        
        let lessons = self.lesson_manager.blocking_lock().get_all_lessons();
        let progress = self.progress_manager.blocking_lock().get_progress();
        
        for (i, category) in self.categories.iter().enumerate() {
            let category_lessons = self.lesson_manager.blocking_lock()
                .get_lessons_by_category(category);
            
            let completed_in_category: usize = category_lessons.iter()
                .filter(|l| progress.completed_lessons.contains_key(&l.id))
                .filter(|l| progress.completed_lessons.get(&l.id).map(|p| p.completed).unwrap_or(false))
                .count();
            
            let status = if i == self.current_category_index {
                "▶ "
            } else {
                "  "
            };
            
            let item = format!(
                "{}{} ({}/{} completed)",
                status,
                category.display_name(),
                completed_in_category,
                category_lessons.len()
            );
            
            list_items.push(ListItem::new(item));
        }
        
        let list = List::new(list_items)
            .block(Block::default().title("Categories"))
            .highlight_style(Style::default().fg(Color::Green).bold());
        
        f.render_widget(list, inner[1]);
        
        let help = Paragraph::new("↑/↓: Navigate | Enter: Select | q: Quit")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, inner[2]);
    }

    fn render_lesson(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        let lessons = self.lesson_manager.blocking_lock()
            .get_lessons_by_category(&self.categories[self.current_category_index]);
        
        let lesson = lessons.get(self.current_lesson_index);
        
        if let Some(lesson) = lesson {
            let block = Block::default()
                .title(format!(" {} ", lesson.title))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));
            
            f.render_widget(block, size);
            
            let inner = Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2);
            
            let mut lines = Vec::new();
            
            for block in &lesson.content {
                match block {
                    ContentBlock::Header(text) => {
                        lines.push(Line::from(Span::styled(text.clone(), Style::default().bold().fg(Color::Yellow))));
                        lines.push(Line::from(""));
                    }
                    ContentBlock::Text(text) => {
                        for line in text.lines().take(3) {
                            lines.push(Line::from(line));
                        }
                    }
                    ContentBlock::Code { code, expected_output, .. } => {
                        lines.push(Line::from(Span::styled(
                            format!("$ {}", code),
                            Style::default().fg(Color::Green),
                        )));
                        if let Some(output) = expected_output {
                            for line in output.lines().take(3) {
                                lines.push(Line::from(Span::styled(
                                    format!("   {}", line),
                                    Style::default().fg(Color::Blue),
                                )));
                            }
                        }
                        lines.push(Line::from(""));
                    }
                    ContentBlock::Tip(text) => {
                        lines.push(Line::from(Span::styled(
                            format!("💡 Tip: {}", text),
                            Style::default().fg(Color::Yellow),
                        )));
                    }
                    ContentBlock::Note(text) => {
                        lines.push(Line::from(Span::styled(
                            format!("📝 Note: {}", text),
                            Style::default().fg(Color::Magenta),
                        )));
                    }
                }
            }
            
            if !lesson.exercises.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Press Enter to start exercises →",
                    Style::default().fg(Color::Green).bold(),
                )));
            }
            
            let paragraph = Paragraph::new(lines)
                .scroll((self.lesson_scroll, 0))
                .wrap(ratatui::widgets::Wrap { trim: false });
            
            f.render_widget(paragraph, inner);
            
            if lines.len() > (inner.height as usize) {
                let scrollbar = Scrollbar::default()
                    .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                    .track_symbol(Some("│"))
                    .thumb_symbol("█")
                    .style(Style::default().fg(Color::Gray));
                
                f.render_stateful_widget(
                    scrollbar,
                    inner,
                    &mut ScrollbarState::new(lines.len().saturating_sub(inner.height as usize))
                        .position(self.lesson_scroll as usize),
                );
            }
        }
    }

    fn render_exercise(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        let lessons = self.lesson_manager.blocking_lock()
            .get_lessons_by_category(&self.categories[self.current_category_index]);
        
        let lesson = match lessons.get(self.current_lesson_index) {
            Some(l) => l,
            None => return,
        };
        
        let exercise = lesson.exercises.get(self.current_exercise_index);
        
        let block = Block::default()
            .title(format!(" Exercise: {} ", lesson.title))
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        
        f.render_widget(block, size);
        
        let inner = Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(inner);
        
        let desc = Paragraph::new(exercise.map(|e| e.description.as_str()).unwrap_or("No exercises"))
            .style(Style::default().fg(Color::White));
        f.render_widget(desc, chunks[0]);
        
        let code_preview = Paragraph::new(
            exercise.map(|e| format!("$ {}", e.initial_code.clone() + "\n> " + &self.input_buffer))
                .unwrap_or_default()
        )
        .style(Style::default().fg(Color::Green));
        f.render_widget(code_preview, chunks[1]);
        
        let output_block = Block::default()
            .title(" Output ")
            .borders(Borders::ALL)
            .style(Style::default().fg(if self.last_error.is_empty() { Color::White } else { Color::Red }));
        
        let output_text = if !self.last_error.is_empty() {
            self.last_error.clone()
        } else if !self.last_output.is_empty() {
            self.last_output.clone()
        } else {
            "Run a command to see output...".to_string()
        };
        
        let output = Paragraph::new(output_text)
            .block(output_block)
            .scroll((self.output_scroll, 0))
            .wrap(ratatui::widgets::Wrap { trim: false });
        
        f.render_widget(output, chunks[2]);
        
        let hint_text = if self.show_hints && !exercise.map(|e| e.hints.is_empty()).unwrap_or(true) {
            let idx = (self.hints_revealed as usize).min(exercise.map(|e| e.hints.len()).unwrap_or(0).saturating_sub(1));
            exercise.and_then(|e| e.hints.get(idx)).cloned().unwrap_or_default()
        } else if self.show_hints {
            "No hints available for this exercise.".to_string()
        } else {
            "Press 'r' to reveal hints".to_string()
        };
        
        let hints = Paragraph::new(hint_text)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(hints, chunks[3]);
    }

    fn render_completed(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        let block = Block::default()
            .title(" Exercise Complete! ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));
        
        f.render_widget(block, size);
        
        let inner = Rect::new(size.x + 1, size.y + 1, size.width - 2, size.height - 2);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(inner);
        
        let title = Paragraph::new("🎉 Congratulations!")
            .style(Style::default().fg(Color::Yellow).bold().centered());
        f.render_widget(title, chunks[0]);
        
        let xp_text = format!(
            "You earned {} XP!",
            self.xp_earned.unwrap_or(0)
        );
        let xp = Paragraph::new(xp_text)
            .style(Style::default().fg(Color::Green).bold().centered());
        f.render_widget(xp, chunks[1]);
        
        let next = Paragraph::new("Press Enter for next lesson or Esc to return to menu")
            .style(Style::default().fg(Color::DarkGray).centered());
        f.render_widget(next, chunks[3]);
    }

    fn render_status_bar(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        let progress = self.progress_manager.blocking_lock().get_progress();
        
        let status_text = format!(
            " XP: {} | Streak: {} days | Completed: {} | {} ",
            progress.total_xp,
            progress.current_streak,
            progress.completed_lessons.values().filter(|p| p.completed).count(),
            match self.current_screen {
                Screen::Menu => "Menu",
                Screen::Lesson => "Lesson",
                Screen::Exercise => "Exercise",
                Screen::Completed => "Completed!",
            }
        );
        
        let status = Paragraph::new(status_text)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center);
        
        f.render_widget(status, Rect::new(size.x, size.height - 1, size.width, size.height));
    }

    fn reveal_hint(&mut self) {
        self.show_hints = true;
        self.hints_revealed += 1;
    }

    async fn run_exercise(&mut self) {
        let lessons = self.lesson_manager.blocking_lock()
            .get_lessons_by_category(&self.categories[self.current_category_index]);
        
        let lesson = match lessons.get(self.current_lesson_index) {
            Some(l) => l,
            None => return,
        };
        
        let exercise = match lesson.exercises.get(self.current_exercise_index) {
            Some(e) => e,
            None => return,
        };
        
        let code = if self.input_buffer.is_empty() {
            exercise.initial_code.clone()
        } else {
            self.input_buffer.clone()
        };
        
        let executor = NuExecutor::new();
        
        let validation: Result<ValidationResult, anyhow::Error> = executor.validate_output(&code, exercise.expected_output.as_deref().unwrap_or("")).await;
        
        match validation {
            Ok(result) => {
                if result.matches_exact || result.matches_partial {
                    let hints_used = self.hints_revealed;
                    let first_try = self.input_buffer.is_empty();
                    
                    let xp: u32 = self.progress_manager.blocking_lock()
                        .complete_lesson(&lesson.id, hints_used, first_try)
                        .unwrap_or(0);
                    
                    self.xp_earned = Some(xp);
                    self.completed_exercise = true;
                    self.current_screen = Screen::Completed;
                    self.last_output = result.result.stdout;
                } else {
                    self.last_output = result.result.stdout;
                    self.last_error = if !result.result.stderr.is_empty() {
                        result.result.stderr
                    } else {
                        "Output doesn't match expected result. Try again or press 'r' for hints.".to_string()
                    };
                }
            }
            Err(e) => {
                self.last_error = e.to_string();
            }
        }
    }

    fn go_to_next_lesson(&mut self) {
        let lessons = self.lesson_manager.blocking_lock()
            .get_lessons_by_category(&self.categories[self.current_category_index]);
        
        if self.current_lesson_index + 1 < lessons.len() {
            self.current_lesson_index += 1;
            self.reset_state();
            self.current_screen = Screen::Lesson;
        } else if self.current_category_index + 1 < self.categories.len() {
            self.current_category_index += 1;
            self.current_lesson_index = 0;
            self.reset_state();
            self.current_screen = Screen::Lesson;
        } else {
            self.current_screen = Screen::Menu;
            self.reset_state();
        }
    }

    fn reset_state(&mut self) {
        self.input_buffer.clear();
        self.hints_revealed = 0;
        self.show_hints = false;
        self.last_output.clear();
        self.last_error.clear();
        self.completed_exercise = false;
        self.xp_earned = None;
        self.lesson_scroll = 0;
        self.output_scroll = 0;
    }
}
