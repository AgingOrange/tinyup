//! # tinyup
//!
//! `tinyup` is a small utility that prints the uptime of the system in a human-readable format.
use anyhow::Result;
use chrono::prelude::*;
use std::process::Command;
use std::time::Duration;

/// Formats a `std::time::Duration` in a human-readable format.
///
/// # Examples
///
/// ```
/// let duration = Duration::from_secs(3661);
/// assert_eq!(format_duration(duration), "01:01");
///
/// let duration = Duration::from_secs(86461);
/// assert_eq!(format_duration(duration), "1d:00:01");
/// ```
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let days = total_seconds / 86400;
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;

    if days > 0 {
        format!("{}d:{:02}:{:02}", days, hours, minutes)
    } else {
        format!("{:02}:{:02}", hours, minutes)
    }
}

/// Fetches the system uptime and formats it in a human-readable format.
fn main() -> Result<()> {
    let output = app()?;
    println!("{}", output);
    Ok(())
}

fn app() -> Result<String> {
    let cmd_output = Command::new("uptime")
        .arg("--since")
        .output()
        .expect("Failed to execute command");
    let cmd_output = String::from_utf8(cmd_output.stdout)?.trim().to_string();

    let dt_start = Local.datetime_from_str(&cmd_output, "%Y-%m-%d %H:%M:%S")?;
    let dt_end = Local::now();
    let duration = dt_end.signed_duration_since(dt_start);
    Ok(format_duration(Duration::from_secs(duration.num_seconds() as u64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        let duration = Duration::from_secs(3661);
        assert_eq!(format_duration(duration), "01:01");

        let duration = Duration::from_secs(86461);
        assert_eq!(format_duration(duration), "1d:00:01");
    }

    #[test]
    fn test_app() {
        let output = app().unwrap();
        assert!(output.len() > 0);
    }
}

