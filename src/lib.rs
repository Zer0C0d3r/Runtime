use colored::*;
use std::fmt::Display;

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
        let uptime_fancy = self.format_uptime_fancy();
        let load_fancy = self.format_load_fancy();
        let user_count = self.system.user_count();
        let boot_time = self.system.boot_time();
        let boot_datetime = chrono::DateTime::from_timestamp(boot_time as i64, 0)
            .unwrap_or_default()
            .with_timezone(&chrono::Local);
        let current_time = chrono::Local::now();
        let icon = "".bright_cyan().bold();
        let time_icon = "".bright_yellow().bold();
        let uptime_icon = "".bright_green().bold();
        let boot_icon = "".bright_magenta().bold();
        let user_icon = "".bright_blue().bold();
        let load_icon = "".bright_red().bold();
        let border = "─".repeat(40).bright_blue().bold();
        let mode_str = if self.system.in_container() {
            "󰆧 Container".bright_cyan().bold()
        } else {
            " Native".bright_green().bold()
        };
        format!(
            "\n{} {} SYSTEM UPTIME {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{}\n",
            border,
            icon,
            border,
            time_icon,
            "Time:",
            current_time
                .format("%H:%M:%S %Z")
                .to_string()
                .bright_white()
                .bold(),
            uptime_icon,
            "Uptime:",
            uptime_fancy,
            boot_icon,
            "Boot:",
            boot_datetime
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .bright_white()
                .bold(),
            user_icon,
            "Users:",
            user_count.to_string().bright_cyan().bold(),
            load_icon,
            "Load:",
            load_fancy,
            mode_str
        )
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.args.format {
            OutputFormat::Raw => {
                // Match uptime's --raw: <current time> <time since boot> <logged in users> <load averages>
                let icon = "".bright_cyan().bold();
                let current_time = chrono::Utc::now().timestamp();
                let uptime_secs = self.system.uptime_seconds();
                let user_count = self.system.user_count();
                let (load1, load5, load15) = self.system.load_averages();
                write!(
                    f,
                    "{} {} {} {} {:.2} {:.2} {:.2}",
                    icon,
                    current_time.to_string().bright_white().bold(),
                    uptime_secs.to_string().bright_yellow().bold(),
                    user_count.to_string().bright_green().bold(),
                    load1,
                    load5,
                    load15
                )
            }
            OutputFormat::Pretty => {
                // Nerd font icon: 
                let icon = "".bright_cyan().bold();
                let uptime_secs = self.system.uptime_seconds();
                let hours = uptime_secs / 3600.0;
                let minutes = (uptime_secs % 3600.0) / 60.0;
                if hours >= 1.0 {
                    let h = hours as u64;
                    let m = minutes as u64;
                    if m > 0 {
                        write!(
                            f,
                            "{} up {} hour{}, {} minute{}",
                            icon,
                            h.to_string().bright_yellow().bold(),
                            if h != 1 { "s" } else { "" },
                            m.to_string().bright_yellow().bold(),
                            if m != 1 { "s" } else { "" }
                        )
                    } else {
                        write!(
                            f,
                            "{} up {} hour{}",
                            icon,
                            h.to_string().bright_yellow().bold(),
                            if h != 1 { "s" } else { "" }
                        )
                    }
                } else {
                    let m = minutes as u64;
                    if m > 0 {
                        write!(
                            f,
                            "{} up {} minute{}",
                            icon,
                            m.to_string().bright_yellow().bold(),
                            if m != 1 { "s" } else { "" }
                        )
                    } else {
                        write!(f, "{} up less than a minute", icon)
                    }
                }
            }
            OutputFormat::Since => {
                // Nerd font icon: 
                let icon = if self.system.in_container() {
                    "󰆧 ".bright_cyan().bold()
                } else {
                    " ".bright_green().bold()
                };
                let datetime = chrono::DateTime::from_timestamp(self.system.boot_time() as i64, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local);
                write!(
                    f,
                    "{} {}",
                    icon,
                    datetime
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .bright_white()
                        .bold()
                )
            }
            OutputFormat::Standard => {
                // Nerd font icon: 
                let icon = "".bright_cyan().bold();
                let now = chrono::Local::now();
                let time_str = now.format("%H:%M:%S").to_string().bright_white().bold();
                let uptime_secs = self.system.uptime_seconds();
                let days = uptime_secs as u64 / 86400;
                let hours = (uptime_secs as u64 % 86400) / 3600;
                let minutes = (uptime_secs as u64 % 3600) / 60;
                let user_count = self.system.user_count();
                let user_str = if user_count == 1 { "user" } else { "users" };
                let (load1, load5, load15) = self.system.load_averages();
                let load_str = format!("{:.2}, {:.2}, {:.2}", load1, load5, load15);
                let container_icon = if self.args.show_container && self.system.in_container() {
                    "󰆧 ".bright_magenta().bold().to_string()
                } else {
                    "".to_string()
                };
                let uptime_str = if days > 0 {
                    format!(
                        "{} day{}, {} min",
                        days.to_string().bright_yellow().bold(),
                        if days != 1 { "s" } else { "" },
                        minutes.to_string().bright_yellow().bold()
                    )
                } else if hours > 0 {
                    format!(
                        "{} min",
                        (hours * 60 + minutes).to_string().bright_yellow().bold()
                    )
                } else {
                    format!("{} min", minutes.to_string().bright_yellow().bold())
                };
                write!(
                    f,
                    "{} {} {}up {}{},  {} {},  load average: {}",
                    icon,
                    container_icon,
                    time_str,
                    uptime_str,
                    if self.args.show_container && self.system.in_container() {
                        " (container)"
                    } else {
                        ""
                    },
                    user_count.to_string().bright_green().bold(),
                    user_str.bright_green().bold(),
                    load_str
                )
            }
            OutputFormat::Interactive => {
                // Minimal, icon-rich, colored dashboard
                let icon = "".bright_cyan().bold();
                let time_icon = "".bright_yellow().bold();
                let uptime_icon = "".bright_green().bold();
                let boot_icon = "".bright_magenta().bold();
                let user_icon = "".bright_blue().bold();
                let load_icon = "".bright_red().bold();
                let border = "─".repeat(40).bright_blue().bold();
                let uptime_fancy = self.format_uptime_fancy();
                let load_fancy = self.format_load_fancy();
                let user_count = self.system.user_count();
                let boot_time = self.system.boot_time();
                let boot_datetime = chrono::DateTime::from_timestamp(boot_time as i64, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local);
                let current_time = chrono::Local::now();
                write!(
                    f,
                    "\n{} {} SYSTEM UPTIME {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {} {}\n",
                    border,
                    icon,
                    border,
                    time_icon,
                    "Time:",
                    current_time.format("%H:%M:%S %Z").to_string().bright_white().bold(),
                    uptime_icon,
                    "Uptime:",
                    uptime_fancy,
                    boot_icon,
                    "Boot:",
                    boot_datetime.format("%Y-%m-%d %H:%M:%S").to_string().bright_white().bold(),
                    user_icon,
                    "Users:",
                    user_count.to_string().bright_cyan().bold(),
                    load_icon,
                    "Load:",
                    load_fancy,
                    if self.system.in_container() { "󰆧 Container".bright_cyan().bold() } else { " Native".bright_green().bold() }
                )
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
