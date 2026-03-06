use crate::executor::NuExecutor;
use anyhow::Result;

pub struct HintGenerator {
    executor: NuExecutor,
}

impl HintGenerator {
    pub fn new() -> Self {
        Self {
            executor: NuExecutor::new(),
        }
    }

    pub fn generate_hints(&self, code: &str, expected: &str) -> Vec<String> {
        let mut hints = Vec::new();
        
        let command_type = self.analyze_command(code);
        
        hints.push(format!(
            "Hint 1: This exercise is about {}. Try running the command first.",
            command_type
        ));
        
        if expected.contains("table") || expected.contains("╭") {
            hints.push("Hint 2: The expected output is a table. Make sure you're using the correct command that produces structured output.".to_string());
        }
        
        if expected.contains('|') {
            hints.push("Hint 3: This exercise involves pipelines. Connect multiple commands using the `|` operator.".to_string());
        }
        
        hints.push(format!(
            "Solution: The expected command is: {}",
            expected.lines().next().unwrap_or(code)
        ));
        
        hints
    }

    fn analyze_command(&self, code: &str) -> &str {
        let cmd = code.trim().split_whitespace().next().unwrap_or("");
        
        match cmd {
            "ls" => "listing directory contents",
            "ps" => "viewing processes",
            "ls" | "ps" | "date" | "whoami" => "basic shell commands",
            "where" => "filtering data",
            "sort-by" => "sorting data",
            "get" => "extracting values",
            "each" => "iterating over data",
            "reduce" => "reducing data to a single value",
            "let" | "mut" | "const" => "working with variables",
            "if" | "match" => "control flow",
            _ => "nushell commands",
        }
    }

    pub async fn provide_error_hint(&self, code: &str, error: &str) -> String {
        let error_lower = error.to_lowercase();
        
        if error_lower.contains("unknown command") {
            format!("Unknown command error: Check if you spelled the command correctly. Type `help` to see available commands.")
        } else if error_lower.contains("type mismatch") || error_lower.contains("type error") {
            "Type mismatch error: You're trying to use a value of the wrong type. Check the data types in your command.".to_string()
        } else if error_lower.contains("column not found") {
            "Column not found: The column you're trying to access doesn't exist. Use `describe` to see available columns.".to_string()
        } else if error_lower.contains("parser error") {
            "Parser error: There's a syntax issue. Check your brackets, quotes, and operators.".to_string()
        } else {
            format!("Error: {}. Try reviewing the lesson content and checking your command syntax.", error.lines().next().unwrap_or("Unknown error"))
        }
    }
}

impl Default for HintGenerator {
    fn default() -> Self {
        Self::new()
    }
}
