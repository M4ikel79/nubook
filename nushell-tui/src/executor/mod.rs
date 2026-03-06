use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncReadExt};

pub struct NuExecutor {
    nu_path: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

impl NuExecutor {
    pub fn new() -> Self {
        let nu_path = std::env::var("NU_PATH").unwrap_or_else(|_| "nu".to_string());
        Self { nu_path }
    }

    pub async fn execute(&self, code: &str) -> Result<ExecutionResult> {
        let mut child = Command::new(&self.nu_path)
            .arg("-c")
            .arg(code)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn nushell process")?;

        let mut stdout: String = String::new();
        let mut stderr: String = String::new();

        if let Some(mut out) = child.stdout.take() {
            out.read_to_string(&mut stdout).await?;
        }
        if let Some(mut err) = child.stderr.take() {
            err.read_to_string(&mut stderr).await?;
        }

        let status: std::process::ExitStatus = child.wait().await?;
        let exit_code: i32 = status.code().unwrap_or(-1);

        Ok(ExecutionResult {
            stdout,
            stderr,
            exit_code,
            success: status.success(),
        })
    }

    pub async fn validate_output(&self, code: &str, expected: &str) -> Result<ValidationResult> {
        let result = self.execute(code).await?;
        
        let normalized_expected = normalize_output(expected);
        let normalized_actual = normalize_output(&result.stdout);
        
        let matches_exact = normalized_actual == normalized_expected;
        let matches_partial = normalized_expected.is_empty() || 
            normalized_actual.contains(&normalized_expected);
        
        Ok(ValidationResult {
            result,
            matches_exact,
            matches_partial,
        })
    }
}

fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub struct ValidationResult {
    pub result: ExecutionResult,
    pub matches_exact: bool,
    pub matches_partial: bool,
}

impl Default for NuExecutor {
    fn default() -> Self {
        Self::new()
    }
}
