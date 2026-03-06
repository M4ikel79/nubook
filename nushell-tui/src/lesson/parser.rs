use super::{ContentBlock, Exercise, Lesson, LessonCategory};
use anyhow::Result;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct LessonParser {
    nubook_path: String,
    code_block_regex: Regex,
    output_regex: Regex,
}

impl LessonParser {
    pub fn new(nubook_path: &str) -> Self {
        Self {
            nubook_path: nubook_path.to_string(),
            code_block_regex: Regex::new(r#"# => (.+)"#).unwrap(),
            output_regex: Regex::new(r"(?m)^# => (.+)$").unwrap(),
        }
    }

    pub fn parse_all(
        &self,
    ) -> Result<(
        HashMap<String, Lesson>,
        HashMap<LessonCategory, Vec<String>>,
    )> {
        let mut lessons = HashMap::new();
        let mut categories: HashMap<LessonCategory, Vec<String>> = HashMap::new();

        let nubook_dir = Path::new(&self.nubook_path);

        self.scan_directory(nubook_dir, &mut lessons, &mut categories)?;

        for category in categories.values_mut() {
            category.sort_by(|a, b| {
                let a_num: u32 = a.split('-').next().unwrap_or("0").parse().unwrap_or(0);
                let b_num: u32 = b.split('-').next().unwrap_or("0").parse().unwrap_or(0);
                a_num.cmp(&b_num)
            });
        }

        Ok((lessons, categories))
    }

    fn scan_directory(
        &self,
        dir: &Path,
        lessons: &mut HashMap<String, Lesson>,
        categories: &mut HashMap<LessonCategory, Vec<String>>,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_directory(&path, lessons, categories)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(lesson) = self.parse_lesson(&path)? {
                    let category = lesson.category.clone();
                    let id = lesson.id.clone();

                    categories.entry(category).or_default().push(id.clone());
                    lessons.insert(id, lesson);
                }
            }
        }
        Ok(())
    }

    fn parse_lesson(&self, path: &Path) -> Result<Option<Lesson>> {
        let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

        let number_regex = Regex::new(r"^(\d+)")?;
        let number: u32 = number_regex
            .captures(filename)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);

        if number == 0 {
            return Ok(None);
        }

        let content = fs::read_to_string(path)?;
        let mut blocks = Vec::new();
        let mut exercises = Vec::new();

        let parser = Parser::new_ext(&content, Options::all());
        let mut in_code_block = false;
        let mut code_language = String::new();
        let mut code_buffer = Vec::new();
        let mut in_header = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { .. }) => {
                    in_header = true;
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_header = false;
                }
                Event::Start(Tag::CodeBlock(lang)) => {
                    in_code_block = true;
                    code_language = lang.0.unwrap_or_default();
                    code_buffer.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let code = code_buffer.join("\n");
                    let expected_output = self.extract_expected_output(&code);
                    let code_for_validation = self.strip_output_markers(&code);

                    if code_language == "nu" && !code_for_validation.trim().is_empty() {
                        exercises.push(Exercise {
                            id: format!("{}-{}", number, exercises.len()),
                            description: "Run this command and verify the output".to_string(),
                            initial_code: code_for_validation,
                            expected_output,
                            hints: vec![],
                            validation_pattern: None,
                        });
                    }

                    blocks.push(ContentBlock::Code {
                        language: code_language.clone(),
                        code: code_for_validation,
                        expected_output,
                    });
                    code_language.clear();
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_buffer.push(text.to_string());
                    } else if in_header {
                        blocks.push(ContentBlock::Header(text.to_string()));
                    } else {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            if trimmed.starts_with("> **") {
                                if trimmed.contains("Tips") {
                                    blocks.push(ContentBlock::Tip(
                                        trimmed.replace("> **Tips**", "").trim().to_string(),
                                    ));
                                } else if trimmed.contains("Note") {
                                    blocks.push(ContentBlock::Note(
                                        trimmed.replace("> **Note**", "").trim().to_string(),
                                    ));
                                }
                            } else {
                                blocks.push(ContentBlock::Text(text.to_string()));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let title = self.extract_title(&blocks);
        let category = LessonCategory::from_number(number);

        Ok(Some(Lesson {
            id: format!("{:03}-{}", number, title.to_lowercase().replace(' ', "-")),
            number,
            title,
            category,
            content: blocks,
            exercises,
        }))
    }

    fn extract_title(&self, blocks: &[ContentBlock]) -> String {
        for block in blocks {
            if let ContentBlock::Header(title) = block {
                return title.clone();
            }
        }
        "Untitled".to_string()
    }

    fn extract_expected_output(&self, code: &str) -> Option<String> {
        let mut outputs = Vec::new();

        for cap in self.output_regex.captures_iter(code) {
            outputs.push(
                cap.get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
            );
        }

        if outputs.is_empty() {
            None
        } else {
            Some(outputs.join("\n"))
        }
    }

    fn strip_output_markers(&self, code: &str) -> String {
        let mut result = String::new();
        for line in code.lines() {
            if !line.starts_with("# => ") {
                result.push_str(line);
                result.push('\n');
            }
        }
        result.trim().to_string()
    }
}
