use anyhow::Result;
use chrono::prelude::*;
use std::fmt;
use std::process::{Command, Stdio};

const DAY_IN_SECONDS: i64 = 86400;
const HOUR_IN_SECONDS: i64 = 3600;
const MINUTE_IN_SECONDS: i64 = 60;

struct Duration {
    num_days: i64,
    num_hours: i64,
    num_minutes: i64,
}

impl Duration {
    const fn new(seconds: i64) -> Self {
        let num_days = seconds / DAY_IN_SECONDS;
        let num_hours = (seconds % DAY_IN_SECONDS) / HOUR_IN_SECONDS;
        let num_minutes = (seconds % HOUR_IN_SECONDS) / MINUTE_IN_SECONDS;
        Self {
            num_days,
            num_hours,
            num_minutes,
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_days > 0 {
            write!(f, "{}d:", self.num_days)?;
        }

        write!(f, "{:02}:{:02}", self.num_hours, self.num_minutes)
    }
}

fn main() {
    match app() {
        Ok(x) => println!("{}", x),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn app() -> Result<String> {
    let cmd_output = Command::new("uptime")
        .arg("--since")
        .stdout(Stdio::piped())
        .output()?;
    let cmd_output = String::from_utf8(cmd_output.stdout)?.trim().to_string();

    let dt_start = Local.datetime_from_str(&cmd_output, "%Y-%m-%d %H:%M:%S")?;
    let dt_end = Local::now();
    let duration = dt_end.signed_duration_since(dt_start);
    Ok(Duration::new(duration.num_seconds()).to_string())
}
