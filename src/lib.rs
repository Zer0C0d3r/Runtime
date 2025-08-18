use colored::*;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod system_metrics;
use system_metrics::SystemMetrics;

/// Runtime structure that holds system metrics and formatting options
#[derive(Debug, Clone)]
pub struct Runtime {
    args: RuntimeArgs,
    system: SystemMetrics,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(RuntimeArgs::default())
    }
}

impl PartialEq for Runtime {
    fn eq(&self, other: &Self) -> bool {
        self.system == other.system
            && self.args.format == other.args.format
            && self.args.show_container == other.args.show_container
    }
}

impl Runtime {
    /// Creates a new Runtime instance
    pub fn new(args: RuntimeArgs) -> Runtime {
        Self {
            args,
            system: SystemMetrics::new().unwrap_or_default(),
        }
    }

    /// Refreshes system metrics
    pub fn refresh(&mut self) {
        if let Ok(()) = self.system.refresh() {
            // Metrics refreshed successfully
        }
    }

    /// Get system uptime as a nicely formatted string with colors
    fn format_uptime_fancy(&self) -> String {
        let uptime_secs = self.system.uptime_seconds();
        let days = uptime_secs as u64 / 86400;
        let hours = (uptime_secs as u64 % 86400) / 3600;
        let minutes = (uptime_secs as u64 % 3600) / 60;
        let seconds = uptime_secs as u64 % 60;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{}d", days.to_string().bright_cyan().bold()));
        }
        if hours > 0 {
            parts.push(format!("{}h", hours.to_string().bright_green().bold()));
        }
        if minutes > 0 {
            parts.push(format!("{}m", minutes.to_string().bright_yellow().bold()));
        }
        if seconds > 0 || parts.is_empty() {
            parts.push(format!("{}s", seconds.to_string().bright_magenta().bold()));
        }

        parts.join(" ")
    }

    /// Get load average with color coding based on system load
    fn format_load_fancy(&self) -> String {
        let (load1, load5, load15) = self.system.load_averages();

        let color_load = |load: f64| {
            if load < 1.0 {
                format!("{:.2}", load).bright_green().bold()
            } else if load < 2.0 {
                format!("{:.2}", load).bright_yellow().bold()
            } else if load < 4.0 {
                format!("{:.2}", load).bright_red().bold()
            } else {
                format!("{:.2}", load).red().bold()
            }
        };

        format!(
            "{}, {}, {}",
            color_load(load1),
            color_load(load5),
            color_load(load15)
        )
    }

    /// Create a clean table layout without nerd fonts
    fn create_table(&self) -> String {
        let border = "=".repeat(55).bright_blue().bold();
        let uptime_fancy = self.format_uptime_fancy();
        let load_fancy = self.format_load_fancy();
        let user_count = self.system.user_count();

        let boot_time = self.system.boot_time();
        let boot_datetime = chrono::DateTime::from_timestamp(boot_time as i64, 0)
            .unwrap_or_default()
            .with_timezone(&chrono::Local);

        let current_time = chrono::Local::now();

        let container_status = if self.args.show_container {
            "[CONTAINER]".bright_cyan().bold().to_string()
        } else {
            "[NATIVE]".bright_green().bold().to_string()
        };

        format!(
            r#"
+{}+
| {}  SYSTEM UPTIME DASHBOARD  {} |
+{}+
| Current Time    : {}               |
| System Uptime   : {}                        |
| Boot Time       : {}        |
| Active Users    : {} {}                      |
| Load Average    : {}               |
| System Mode     : {}               |
+{}+
"#,
            border,
            "*".bright_yellow(),
            "*".bright_yellow(),
            "=".repeat(55).bright_blue().bold(),
            current_time
                .format("%H:%M:%S %Z")
                .to_string()
                .bright_white()
                .bold(),
            uptime_fancy,
            boot_datetime
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .bright_white()
                .bold(),
            user_count.to_string().bright_cyan().bold(),
            if user_count == 1 { "user" } else { "users" }.dimmed(),
            load_fancy,
            container_status,
            "=".repeat(55).bright_blue().bold()
        )
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.args.format {
            OutputFormat::Raw => {
                // Format: boot_time uptime_seconds idle_time load1 load5 load15
                let boot_time = self.system.boot_time();
                let uptime_secs = self.system.uptime_seconds();
                let idle_time = self.system.idle_time();
                let (load1, load5, load15) = self.system.load_averages();

                write!(
                    f,
                    "{} {:.6} {} {:.2} {:.2} {:.2}",
                    boot_time, uptime_secs, idle_time, load1, load5, load15
                )
            }
            OutputFormat::Pretty => {
                // Format: "up X hours, Y minutes"
                let uptime_secs = self.system.uptime_seconds();
                let hours = uptime_secs / 3600.0;
                let minutes = (uptime_secs % 3600.0) / 60.0;

                if hours >= 1.0 {
                    let h = hours as u64;
                    let m = minutes as u64;

                    if m > 0 {
                        write!(
                            f,
                            "up {} hour{}, {} minute{}",
                            h,
                            if h != 1 { "s" } else { "" },
                            m,
                            if m != 1 { "s" } else { "" }
                        )
                    } else {
                        write!(f, "up {} hour{}", h, if h != 1 { "s" } else { "" })
                    }
                } else {
                    let m = minutes as u64;
                    if m > 0 {
                        write!(f, "up {} minute{}", m, if m != 1 { "s" } else { "" })
                    } else {
                        write!(f, "up less than a minute")
                    }
                }
            }
            OutputFormat::Since => {
                // Format: "YYYY-MM-DD HH:MM:SS"
                let boot_time = self.system.boot_time();
                let datetime = chrono::DateTime::from_timestamp(boot_time as i64, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local);
                write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S"))
            }
            OutputFormat::Standard => {
                // Standard uptime format with time, uptime, users, load averages
                let now = chrono::Local::now();
                let time_str = now.format("%H:%M:%S");

                let uptime_secs = self.system.uptime_seconds();
                let days = uptime_secs as u64 / 86400;
                let hours = (uptime_secs as u64 % 86400) / 3600;
                let minutes = (uptime_secs as u64 % 3600) / 60;

                let uptime_str = if days > 0 {
                    if hours > 0 {
                        format!("{}:{:02}", days, hours)
                    } else {
                        format!("{} day{}", days, if days != 1 { "s" } else { "" })
                    }
                } else if hours > 0 {
                    format!("{}:{:02}", hours, minutes)
                } else {
                    format!("{} min", minutes)
                };

                let user_count = self.system.user_count();
                let user_str = if user_count == 1 { "user" } else { "users" };

                let (load1, load5, load15) = self.system.load_averages();

                let container_suffix = if self.args.show_container {
                    " (container)"
                } else {
                    ""
                };

                write!(
                    f,
                    " {} up {}{}, {} {}, load average: {:.2}, {:.2}, {:.2}",
                    time_str,
                    uptime_str,
                    container_suffix,
                    user_count,
                    user_str,
                    load1,
                    load5,
                    load15
                )
            }
            OutputFormat::Interactive => {
                // Clean table format without nerd fonts
                write!(f, "{}", self.create_table())
            }
        }
    }
}

/// Output format options
#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    /// Standard uptime format
    Standard,
    /// Human-readable format
    Pretty,
    /// Raw numerical values
    Raw,
    /// Show since timestamp
    Since,
    /// Interactive colorful table format
    Interactive,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Interactive // Default to the interactive format
    }
}

/// Command line arguments structure
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeArgs {
    pub format: OutputFormat,
    pub show_container: bool,
}

impl Default for RuntimeArgs {
    fn default() -> Self {
        Self {
            format: OutputFormat::Interactive,
            show_container: false,
        }
    }
}
